# Spec: terminal UI (`highset-tui`)

Requirements: UX-1…9, AGT-5/6, ORG-7, SRCH-1, PERF-1/2/6/7. ADRs: 0014, 0017.

Design references: **Linear** (speed, keyboard-first, quiet chrome, instant feedback) and **Superset** (one place to watch many agents in parallel) (Q13.8).

## 1. Layout (IDE style, Q9.1)

```
┌ HighSet ─ work › highset ───────────────────────────────── ◷ 2 running · ⚠ 1 waiting · $3.41 today ┐
│ PROJECTS        │ T-0042 Add login with GitHub · in_progress · claude-code/builder │ CONTEXT        │
│ ▾ work          │ ─────────────────────────────────────────────────────────────── │ Spec  ✓        │
│   ▸ highset  3  │ > claude                                                        │ Plan  ✓        │
│   ▸ api      1  │   Reading crates/auth/src/lib.rs…                               │ Packs          │
│ ▾ personal      │   ● Edit src/oauth.rs (+42 −3)                                  │  rust-backend  │
│                 │   ⚠ Permission: run `cargo test -p auth`  [y] allow [n] deny    │ Memory  4      │
│ TASKS (highset) │                                                                 │ MCP  highset,  │
│  ● T-0042  rev  │                                                                 │      github    │
│  ○ T-0043  plan │                                                                 │ Cost  $1.12    │
│  ○ T-0044  inbx │                                                                 │ Tokens 412k    │
├─────────────────┴─────────────────────────────────────────────────────────────────┴────────────────┤
│ j/k move  enter open  s start  m move  r review  a capture  ctrl+k palette  ? help                    │
└─────────────────────────────────────────────────────────────────────────────────────────────────────┘
```

- **Regions:**
  - Sidebar: workspaces/projects tree and the task list of the selected project.
  - Center: the active view.
  - Context panel: details of the selection.
  - Status bar: running agents, waiting count, today's cost, key hints.
- **Responsive:**
  - Below 120 columns, the context panel collapses (toggle with `\`).
  - Below 90 columns, the sidebar collapses (toggle with `|`).
  - Minimum supported size: 80×24.
- **Colors:** the terminal's 16-color ANSI palette by default (Q9.7), so HighSet inherits the user's theme. Semantic roles (accent, muted, ok, warn, error) map to ANSI colors. Optional presets may use 256/truecolor.

## 2. Views (Q9.5)

| # | View | Content |
|---|---|---|
| 1 | Dashboard | All running and waiting sessions across **all** projects (project · task · agent/profile · state · duration · cost); attention queue; tasks by status per project; today/week cost |
| 2 | Kanban | Columns from the project's flow statuses. Cards show number, title, priority, labels, agent badge, blocked flag, dependency marker |
| 3 | Sessions | Session list + live session pane (§4) |
| 4 | Knowledge | Tree by level → kind; preview; audience and level badges; packs tab |
| 5 | Profiles | Profiles list; resolved view with provenance; MCP server registry tab |
| 6 | Inbox | Proposals (memory, knowledge, decision, doc update) with approve/edit/reject |
| 7 | Timeline | Chronological events, filterable by project/task/agent/type |
| 8 | Plugins | Installed plugins, contributions, permissions; add/update/remove |
| — | Diff | Opened from a task or session: file tree + unified/side-by-side diff + gate actions |
| — | Task detail | Body, acceptance criteria, gates, links, sessions, costs, history |
| v0.2 | Knowledge graph | Not in v0.1 |

## 3. Keymap (Q9.2 + note: Vim **and** conventional, both active)

The defaults below are all remappable in `content/keymap.toml`. Conflicts are reported at startup.

| Action | Vim / single-key | Conventional |
|---|---|---|
| Move | `h j k l` | arrows |
| Page | `ctrl+d` / `ctrl+u` | PgDn / PgUp |
| Top / bottom | `g g` / `G` | Home / End |
| Open / back | `enter` / `esc` | enter / esc |
| Cycle panes | `tab` / `shift+tab` | tab |
| Switch views | `1`…`8` | `ctrl+1`…`ctrl+8` where the terminal supports it |
| Command palette | `:` | `ctrl+k` |
| Search | `/` | `ctrl+f` |
| New (context-aware: task, KB item…) | `n` | `ctrl+n` |
| Quick capture to Inbox | `a` | — |
| Start agent on task | `s` | — |
| Move task status | `m`, then pick; `]` / `[` next/prev | — |
| Review / approve gate | `r` | — |
| Open diff | `d` | — |
| Edit in `$EDITOR` | `e` | — |
| Stop session | `x` (confirm) | — |
| Toggle panels | `\` context, `\|` sidebar | — |
| Help | `?` | `F1` |
| Quit | `q` (confirm if sessions are attached; they keep running) | `ctrl+q` |
| Leave embedded terminal focus | `ctrl+]` (configurable) | same |

A **help bar** (which-key style) at the bottom always shows the keys valid in the current context. `?` opens the full list for the context.

## 4. Live session pane

- **PTY sessions:** render with a ratatui terminal widget (e.g. `tui-term`) over a local `vt100::Parser`, fed by the attach snapshot and the streamed bytes. When the pane is focused, **all keys pass through** to the agent except the escape chord (`ctrl+]`). Bracketed paste and mouse passthrough work when the agent requests them.
- **ACP sessions:** a structured view with:
  - messages (Markdown rendered simply);
  - tool calls (collapsed one-liners with status, expandable);
  - the plan (checklist);
  - inline diffs;
  - permission prompts with `y` (allow once), `Y` (always), `n` (deny).

  A composer at the bottom sends follow-up prompts.
- **Composer** (C-009): multi-line input. `@` opens the mention picker (`context-compiler.md` §4); chips show what will be attached and its size; `ctrl+enter` sends.
- **Switching:** palette, `tab` within the session list, or `alt+1…9` for the first nine sessions.

## 5. Architecture and rendering (PERF-1/2)

- **State and effects:** a single `AppState` and an `Action` enum, with a pure `reduce(state, action) -> (state, Vec<Effect>)`. Effects (daemon calls, opening the editor) run in tokio tasks and post actions back.
- **Event loop:** `select!` over crossterm events, daemon notifications and effect results. **No fixed tick.** Redraw only when the state is dirty, coalesced to ≤ 60 fps. Animations (spinners) run only while something is actually running, at ≤ 10 fps.
- **Startup:**
  1. Connect, or auto-start the daemon.
  2. Call `ui.bootstrap`.
  3. Draw the first frame.
  4. Subscribe to events.
  5. Lazy-load the rest.

  Target ≤ 150 ms to the first frame.
- **Large lists** (tasks, KB, timeline) are virtualized; only visible rows are built.
- **Panics:** a panic hook restores the terminal (leaves the alternate screen, disables raw mode) before printing a message that points to the log file.
- **`$EDITOR`:** suspend the TUI (leave the alternate screen), run the editor, then restore and refresh.

## 6. i18n (UX-9)

All strings come from `highset-i18n` keys (`tui-*`), in English and Spanish. The language is chosen with `ui.language` (`auto` → `LANG`). Snapshot tests run in both languages for the main views.

## 7. Testing

- `insta` snapshots with ratatui `TestBackend` at 80×24, 120×40 and 200×60 for every view, in `en` and `es`.
- Reducer unit tests for every action.
- A benchmark harness feeds synthetic events (4 sessions flooding output, 1k tasks) and asserts frame-time budgets.
- **Manual checklist** (documented in the task verification log): iTerm2, Terminal.app, Ghostty or WezTerm, Alacritty or Kitty, inside tmux; mouse on/off; copy/paste.

## Verified facts

_(Builder: terminal widget crate chosen, measured frame times, terminal quirks found.)_
