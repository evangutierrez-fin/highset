---
id: C-002
title: "Keymap, help bar and command palette"
lane: tui
milestone: M2
size: M
status: todo
depends_on: [C-001]
requirements: [UX-2, UX-3, SRCH-1, PERF-7]
open_questions: []
---

# C-002 · Keymap, help bar and command palette

**Lane:** tui · **Milestone:** M2 · **Size:** M · **Depends on:** [C-001](C-001-tui-shell-state-architecture-and-layout.md)

## Goal

Keyboard-first control where Vim users and everyone else both feel at home.

## Read first

- [`specs/tui.md#3-keymap-q92--note-vim-and-conventional-both-active`](../specs/tui.md#3-keymap-q92--note-vim-and-conventional-both-active)
- ADR [0014](../decisions/0014-terminal-ui-ratatui-ide-layout-hybrid-keymap.md)
- PRD requirements: UX-2, UX-3, SRCH-1, PERF-7

## Scope

- Action registry; `keymap.toml` with the tui §3 defaults (Vim/single-key and conventional active together); conflict detection.
- Contextual help bar and `?` overlay.
- Palette (`ctrl+k` and `:`) with nucleo over actions, projects, tasks, sessions, knowledge and profiles, using event-updated caches.

## Acceptance criteria

- [ ] Remapping through `keymap.toml` works; conflicts are reported at startup.
- [ ] The palette filters 50k candidates in ≤ 10 ms per keystroke (bench).
- [ ] Every registered action is reachable from the palette (test enumerating the registry).
- [ ] Help bar snapshots per context.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
