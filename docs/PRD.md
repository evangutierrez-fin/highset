# HighSet: Product Requirements (v0.1)

| | |
|---|---|
| Status | Draft 1, approved for build planning |
| Date | 2026-10-03 |
| Source of intent | [`inputs/questionnaire-2026-10-03.md`](inputs/questionnaire-2026-10-03.md) (cited as `Q<section>.<item>`) |
| Open decisions | [`OPEN-QUESTIONS.md`](OPEN-QUESTIONS.md) |

## 1. Summary

HighSet is a keyboard-first, local-first **command center for working with AI coding agents** from the terminal. It does not compete with agents such as Claude Code, Codex CLI, opencode or Cursor CLI. It **orchestrates** them:

- organizes work into workspaces, projects, tasks and sessions;
- gives every agent the right context from one source of truth;
- defines harnesses (instructions, model, tools, permissions, hooks) once and syncs them to every agent;
- runs agents in parallel in isolated git worktrees;
- guides work with an adaptive spec-driven methodology.

Owner's vision (Q1.3, translated): *"A command center for working with AI agents, where I control the flows, the information given to the AI and the tools, so I can be more efficient in my work."*

HighSet replaces the owner's current use of Superset (Q3.9). Its interaction quality takes after Linear (Q13.8).

## 2. Problems to solve (Q1.4)

| # | Problem | Addressed by |
|---|---|---|
| P1 | Re-explaining the project to every agent; each session starts from zero | Knowledge base, context compiler, packs, MCP server (CTX-*) |
| P2 | Losing track of what each agent did | Sessions with transcript, summary, diff and cost; timeline; journal (AGT-8, MTH-7) |
| P3 | Configuring each CLI separately (MCP, skills, rules, permissions) | Harness profiles synced from one source (HRN-*) |
| P4 | No method: agents improvise and quality varies | Adaptive SDD flows, gates, verification (MTH-*) |
| P5 | Coordinating several agents without conflicts | Worktree per task, parallel sessions, dashboard (AGT-3, UX-4) |
| P6 | Knowledge lost when a session closes | Memory proposals, ADRs, staleness detection (CTX-11, CTX-12) |

## 3. Goals and non-goals

**Goals for v0.1**

- G1. Start an agent on a task, with full context, in under 30 seconds and with no manual copy-paste.
- G2. See every running agent, across projects, in one screen; never miss an agent that is waiting.
- G3. Define instructions, permissions, MCP servers and skills once; Claude Code and Codex receive them automatically.
- G4. Every finished session leaves a searchable record: transcript, summary, diff, cost.
- G5. Work follows Spec → Plan → Implement → Verify by default, with human gates where they matter.
- G6. Fast and light enough that the owner never notices it (budgets in §8).
- G7. Open-source ready: generic defaults, English and Spanish UI, documented standards.

**Non-goals for v0.1**

- Building our own coding agent or agent loop (ADR-0003).
- A cloud service, accounts, or team collaboration features.
- A GUI or web app.
- Windows support (macOS and Linux only, Q1.7).
- Semantic search or embeddings (Q5.8).
- Scheduled automations (Q11.2, v0.2).

## 4. Users

- **Primary: the maintainer.** A power user of several agent CLIs who works on software development, research and analysis, quantitative data work, and automation (Q1.6). Does not read Rust (Q12.1–12.2).
- **Secondary: open-source users** with the same profile (Q1.5). Because the project is public from day 1, defaults must be generic (no personal paths), docs must be good, and the UI ships in English and Spanish (Q2.6).

## 5. Product principles

The first seven come from the owner's ranking (Q13.7):

1. **Context quality first.** When in doubt, optimize what the agent knows.
2. **Organization.** Everything has a place: workspace, project, task, session.
3. **Ship the MVP quickly.** Thin versions of every v0.1 feature beat a perfect subset.
4. **Robust and secure.** No data loss, no secret leaks, no surprise actions.
5. **Extensible.** Plugins and standard formats instead of hardcoded integrations.
6. **Fast and light.** Ranked 6th as a goal, but it is the owner's one stated deal-breaker (Q13.9), so the performance budgets are **hard constraints** (ADR-0017).
7. **Visual polish last**, but keyboard flow is part of the core, not polish.

Additional principles:

- **Standards over invention (Q13.10):** AGENTS.md, Agent Skills, MCP, ACP, Spec Kit layout, MADR, Conventional Commits, SemVer, Keep a Changelog (ADR-0018).
- **Plain files are the truth:** Markdown and TOML, readable and versionable. Databases are caches (ADR-0007).
- **Agent-agnostic:** nothing works only with one vendor's agent.
- **Local-first, no telemetry** (Q10.6).

## 6. Glossary

| Term | Meaning |
|---|---|
| Workspace | A top-level context (e.g. "Work", "Personal", "Client X") that groups projects and can hold shared knowledge and config. |
| Project | A folder or git repo, optionally with linked repos. Holds `.highset/`. |
| Task | A unit of work with a status, a flow, metadata, and at most one worktree/branch. Human ID `T-0001`, unique per project. |
| Session | One run of one agent on one task (or ad hoc), with mode ACP, PTY or headless. Produces a transcript, summary, diff and usage. |
| Agent | An external coding agent CLI: Claude Code, Codex CLI, opencode, Cursor CLI. |
| Adapter | HighSet code that knows how to detect, launch, configure and observe one agent. |
| Profile (harness) | A named agent configuration: instructions, model, MCP servers, skills, commands, permissions, hooks, subagents, packs. Built-ins: researcher, planner, builder, reviewer, debugger, documenter. |
| Knowledge item | A Markdown document (plus optional asset) with frontmatter: kind, tags, audience, source. |
| Level | Where an item lives: global, workspace, project (committed), or local (project-private, not committed). |
| Audience | An optional restriction on which agents, profiles or tasks may see an item (Q5.3 note). |
| Convention file | A file whose name declares its purpose, like `PROJECT.md`, `*.instructions.md`, `*.prompt.md`, `*.agent.md`, `SKILL.md` or `MEMORY.md` (Q5.4 note). |
| Context pack | A reusable bundle of knowledge, instructions, skills, prompts, MCP servers and profiles, enabled per project or task. |
| ContextBundle | The agent-neutral result of compiling context for one (project, task, profile, agent). Emitted to agent-specific files or launch flags. |
| Sync target | An agent-specific output: `CLAUDE.md`, `.claude/settings.json`, `AGENTS.md`, Codex config, and so on. |
| Mention | An `@` reference in a prompt (`@kb:style-guide`, `@task:T-3`, `@file:src/main.rs`) that expands into attached context (Q2.4 note). |
| Flow | A methodology definition: phases, statuses, gates, checks. Built-ins: `feature`, `fix`. |
| Gate | A human approval point: approve spec, approve plan, approve diff. |
| Attention item | Something waiting for the owner: a permission request, input needed, finished session, pending gate, or budget alert. |
| Proposal | A suggested memory, knowledge or decision change, waiting for approval in the inbox. |

## 7. Functional requirements

Release column: **v0.1** = MVP, **v0.2** = next release, **later** = backlog.

### 7.1 Organization (ORG)

| ID | Requirement | Release |
|---|---|---|
| ORG-1 | Workspaces group projects; create, list, rename, remove from CLI and TUI. | v0.1 |
| ORG-2 | A project is a folder or git repo (Q4.2), with optional linked repos. | v0.1 |
| ORG-3 | Tasks are Markdown files with frontmatter in `.highset/tasks/`. Human IDs are `T-0001`, unique per project. | v0.1 |
| ORG-4 | Task statuses come from the task's flow: Inbox → Spec → Plan → In progress → Review → Done, plus Canceled and a Blocked flag (Q4.4). Editable per project. | v0.1 |
| ORG-5 | Task metadata: priority, labels, assigned profile/agent, dependencies, links (files, commits, PRs), accumulated cost (Q4.5). Optional epic. | v0.1 |
| ORG-6 | Quick capture: `highset add "idea"` and a single key in the TUI put an item in the Inbox (Q4.9). Outside a project, it goes to the global inbox. | v0.1 |
| ORG-7 | Kanban board by status, with keyboard moves. | v0.1 |
| ORG-8 | Sync with external trackers (GitHub Issues, Linear…) through plugins (Q4.3). | v0.2 |

### 7.2 Agents and sessions (AGT)

| ID | Requirement | Release |
|---|---|---|
| AGT-1 | Launch Claude Code, Codex CLI, opencode and Cursor CLI on a task (Q3.1). | v0.1 |
| AGT-2 | Integration modes: ACP (preferred), embedded PTY (fallback), headless JSON (automation) (Q3.2). | v0.1 |
| AGT-3 | One git worktree and branch per task; several agents run in parallel on different tasks (Q3.3). | v0.1 |
| AGT-4 | Sessions keep running when the TUI closes; reattach any time (Q2.2). | v0.1 |
| AGT-5 | Live session view: stream output, send input, switch sessions quickly. | v0.1 |
| AGT-6 | Permission requests from agents are shown in the TUI and as OS notifications, with approve/deny. | v0.1 |
| AGT-7 | Default autonomy is semi-autonomous: agents work freely inside their worktree; humans approve at the flow's gates (Q3.6). | v0.1 |
| AGT-8 | Every session stores a full transcript, a summary, the final diff and token usage, all searchable (Q4.6). | v0.1 |
| AGT-9 | Notifications when an agent finishes and when it needs permission or input; summary in the status bar (Q9.6). | v0.1 |
| AGT-10 | Built-in profiles: researcher, planner, builder, reviewer, debugger, documenter (Q3.4). | v0.1 |
| AGT-11 | Additional agents (e.g. Gemini CLI) through adapter plugins. | v0.2 |

### 7.3 Knowledge and context (CTX)

| ID | Requirement | Release |
|---|---|---|
| CTX-1 | Knowledge items are Markdown with frontmatter, covering docs, notes, prompts, specs, transcripts, web pages, PDFs, snippets, images and data (Q5.1–5.2). | v0.1 |
| CTX-2 | Levels: global (`~/.highset`), workspace, project (committed in `.highset/`), and local (project-private, gitignored) (Q5.3). | v0.1 |
| CTX-3 | Audience: an item can be restricted to specific agents, profiles or tasks (Q5.3 note). | v0.1 |
| CTX-4 | Convention files are recognized by name: `PROJECT.md`, `*.instructions.md` (with optional `applyTo` globs), `*.prompt.md`, `*.agent.md`, `skills/<name>/SKILL.md`, `MEMORY.md` (Q5.4 note). | v0.1 |
| CTX-5 | Context packs bundle knowledge, instructions, skills, prompts, MCP servers and profiles, and are enabled per project, task or profile (Q5.5). | v0.1 |
| CTX-6 | Ingestion: files, URL → clean Markdown, PDF → text, images, data files (Q5.6). | v0.1 |
| CTX-6b | Ingestion: repo → knowledge graph via graphify; import from Notion/Obsidian (as plugins). | v0.2 |
| CTX-7 | The context compiler generates `CLAUDE.md` and `AGENTS.md` (plus optional `GEMINI.md` and Cursor rules) from one source, inside managed blocks that preserve user content (Q5.4). | v0.1 |
| CTX-8 | Agents fetch context on demand through HighSet's MCP server (Q3.5, Q5.4). | v0.1 |
| CTX-9 | Starting a task attaches its spec/plan and the task's packs to the initial prompt (Q5.4). | v0.1 |
| CTX-10 | `@`-mentions in prompts expand into attached context: files, knowledge, tasks, specs, packs, sessions, URLs. Fuzzy picker in the TUI (Q2.4 note, ADR-0021). | v0.1 |
| CTX-11 | Memory: at session end, decisions, learnings and todos are proposed; the owner approves before anything is saved (Q5.7). | v0.1 |
| CTX-12 | Staleness: when a task finishes, docs covering the changed files are flagged with a proposed update (Q5.10). | v0.1 |
| CTX-13 | `highset context explain` shows exactly what an agent will see, and why. | v0.1 |
| CTX-14 | Semantic search / embeddings: evaluate only after measuring need. | later |

### 7.4 Harness (HRN)

| ID | Requirement | Release |
|---|---|---|
| HRN-1 | A profile defines instructions, model and parameters, MCP servers, skills, slash commands/prompts, permissions (allow/deny), hooks and subagents (Q6.1). | v0.1 |
| HRN-2 | HighSet is the source of truth and syncs Claude Code and Codex (Q6.2, Q13.1). opencode and Cursor are synced best-effort in v0.1. | v0.1 |
| HRN-3 | Formats: TOML for configuration; Markdown + YAML frontmatter for profiles, prompts and skills, compatible with Agent Skills (Q6.3). | v0.1 |
| HRN-4 | Configuration layers: global → workspace → project → task; each overrides the previous (Q6.4). | v0.1 |
| HRN-5 | Security: each agent's native sandbox plus centralized permission rules compiled into each agent's format (Q6.5). | v0.1 |
| HRN-6 | Hooks on session start/end, task status change, pre-commit/merge, and attention events (Q6.6). | v0.1 |
| HRN-7 | Harness history through git (Q6.7). | v0.1 |
| HRN-7b | A/B comparison of harnesses: same task, two profiles, compare result and cost. | v0.2 |
| HRN-8 | Central MCP server registry: install once, enable per profile/project, synced to every agent; secrets in the keychain (Q7.5). | v0.1 |

### 7.5 Methodology (MTH)

| ID | Requirement | Release |
|---|---|---|
| MTH-1 | Adaptive flows (Q8.1): `feature` = Spec → Plan → Implement → Verify; `fix` = Plan → Implement → Verify. | v0.1 |
| MTH-2 | Gates (Q8.2): approve spec, approve plan, review the diff before merge. | v0.1 |
| MTH-3 | Verification (Q8.3): required tests; lint, format and type checks; the spec's acceptance checklist; review by a second agent (reviewer profile). | v0.1 |
| MTH-4 | Guide mode (Q8.4): HighSet suggests the next step; skipping is allowed and logged with a reason. | v0.1 |
| MTH-5 | Templates (spec, plan, tasks, ADR, PR, retro) are shipped and editable (Q8.6). Spec artifacts use a Spec Kit–compatible layout. | v0.1 |
| MTH-6 | Rituals (Q8.5): a daily standup (what agents did, what awaits review) and a weekly review (progress, costs, learnings). | v0.1 |
| MTH-7 | Logs (Q4.8): ADRs (MADR), a project changelog (Keep a Changelog, from Conventional Commits), a daily work journal. | v0.1 |
| MTH-8 | Git (Q8.7): branch and worktree per task, checkpoint commits at phase transitions, Conventional Commits, a PR at the end (GitHub via `gh`). | v0.1 |

### 7.6 Search (SRCH)

| ID | Requirement | Release |
|---|---|---|
| SRCH-1 | Instant fuzzy search over names, commands, tasks, knowledge and files (command palette). | v0.1 |
| SRCH-2 | Full-text search over tasks, knowledge, specs, transcripts and summaries. | v0.1 |
| SRCH-3 | Metadata filters: status, label, agent, profile, project, kind, date. | v0.1 |

### 7.7 Costs (COST)

| ID | Requirement | Release |
|---|---|---|
| COST-1 | Tokens and cost per session, task, project and day (Q3.8). | v0.1 |
| COST-2 | An editable pricing table with shipped defaults. | v0.1 |
| COST-3 | Subscription mode shows tokens and usage instead of money. | v0.1 |
| COST-4 | Budgets per project or task, with alerts at 80% and 100%; optional pause at 100%. | v0.1 |

### 7.8 Plugins (PLG)

| ID | Requirement | Release |
|---|---|---|
| PLG-1 | Declarative plugin packages contribute skills, prompts, profiles, packs, convention files, MCP servers, hooks, shell commands and translations (Q7.1–7.2). | v0.1 |
| PLG-2 | Install from git (`highset plugin add github:owner/repo`) or local folders. A lockfile pins commits (Q7.4). | v0.1 |
| PLG-3 | Each plugin declares its permissions (network, files, commands, secrets); the owner approves them at install and again on any change (Q7.6). | v0.1 |
| PLG-4 | Claude Code plugins and marketplaces can be imported (Q7.3). | v0.1 |
| PLG-5 | Standalone Agent Skills (`SKILL.md`) can be imported; any MCP server can be registered (Q7.3). | v0.1 |
| PLG-6 | Code plugins as out-of-process programs speaking JSON-RPC over stdio: agent adapters, importers, integrations (Q7.1). | v0.2 |
| PLG-7 | A public plugin registry (Q7.4). | later |

### 7.9 Terminal UX (UX)

| ID | Requirement | Release |
|---|---|---|
| UX-1 | Full-screen ratatui TUI with an IDE-style layout: sidebar, center view, context panel, status bar (Q2.5, Q9.1). | v0.1 |
| UX-2 | Keymap: Vim navigation plus single-key actions with a visible help bar, **and** conventional keys (arrows, Ctrl shortcuts) always active. Fully remappable (Q9.2 + note). | v0.1 |
| UX-3 | Command palette (Ctrl+K and `:`) with fuzzy search over everything (Q9.3). | v0.1 |
| UX-4 | Views (Q9.5) in v0.1: dashboard (all active agents across projects, tasks, costs), kanban, live session, diff viewer, knowledge explorer, profile/harness editor, plugin manager, timeline, and an inbox for proposals. | v0.1 |
| UX-4b | Knowledge graph view. | v0.2 |
| UX-5 | Embedded terminals: no tmux needed (Q9.4). | v0.1 |
| UX-6 | OS notifications on macOS and Linux (Q9.6). | v0.1 |
| UX-7 | Uses the terminal's own color palette by default (Q9.7). | v0.1 |
| UX-8 | Optional mouse support (Q9.8). | v0.1 |
| UX-9 | UI in English and Spanish, chosen automatically from the locale and overridable (Q2.6). | v0.1 |
| UX-10 | Everything the TUI does is also scriptable through the CLI (`--json` output) (Q11.3). | v0.1 |

### 7.10 Data and security (DATA)

| ID | Requirement | Release |
|---|---|---|
| DATA-1 | SQLite as a local, rebuildable cache (Q10.1). | v0.1 |
| DATA-2 | Secrets in the OS keychain (Keychain on macOS, Secret Service on Linux) (Q10.2). | v0.1 |
| DATA-3 | Redact secrets from transcripts, knowledge, memory, MCP outputs and logs before persisting (Q10.3). | v0.1 |
| DATA-4 | Local-first. The content directory (plain files) can be placed in a synced folder (iCloud, Syncthing, a git repo); the state directory (caches) never syncs (Q10.4 note, ADR-0022). | v0.1 |
| DATA-4b | Sync providers as plugins. | v0.2 |
| DATA-5 | Automatic daily snapshot of the content directory with retention (Q10.5). | v0.1 |
| DATA-6 | No telemetry (Q10.6). | v0.1 |

### 7.11 Internal LLM tasks (LLM)

| ID | Requirement | Release |
|---|---|---|
| LLM-1 | Providers: Anthropic, OpenAI, OpenAI-compatible local servers (Ollama, LM Studio) (Q3.7), plus an "agent headless" backend that uses an installed agent CLI, so no API key is required. | v0.1 |
| LLM-2 | Internal tasks (Q2.4): session summary, memory extraction, knowledge tagging, idea → spec draft. Auto-naming of branches, commits and tasks is **not** included (the owner wanted it only if it meant `@`-mentions; mentions are CTX-10). | v0.1 |

### 7.12 Integrations (INT)

| ID | Requirement | Release |
|---|---|---|
| INT-1 | GitHub: PR creation and issue links via the `gh` CLI (Q11.1). | v0.1 |
| INT-2 | Local control API: the daemon socket plus the CLI (Q11.3). | v0.1 |
| INT-3 | Scheduled automations (Q11.2). | v0.2 |

## 8. Non-functional requirements

### 8.1 Performance (release-blocking, ADR-0017)

Measured on an Apple M1-class Mac and a mid-range Linux laptop, with a dataset of 10k tasks and 10k knowledge items. Details in `specs/performance.md`.

| ID | Budget |
|---|---|
| PERF-1 | TUI cold start to first frame (daemon running): ≤ 150 ms p95 |
| PERF-2 | Keypress → rendered frame: ≤ 16 ms p95, including with 4 live sessions streaming |
| PERF-3 | One-shot CLI command (e.g. `highset add`): ≤ 60 ms p95 end to end |
| PERF-4 | Daemon idle CPU: < 0.5% average; no periodic wakeups faster than 1 Hz |
| PERF-5 | Daemon idle RSS: ≤ 60 MB, excluding agent processes |
| PERF-6 | TUI RSS: ≤ 80 MB with 4 attached sessions |
| PERF-7 | Fuzzy palette: ≤ 10 ms per keystroke over 50k candidates |
| PERF-8 | Full-text query: ≤ 50 ms p95 over 10k documents |
| PERF-9 | Full reindex of 10k documents: ≤ 10 s |
| PERF-10 | Release binary: ≤ 40 MB (stripped) |

### 8.2 Other

| ID | Requirement |
|---|---|
| NFR-REL-1 | Crash-safe writes: atomic file replacement; caches always rebuildable from content. |
| NFR-REL-2 | After a daemon restart, sessions that were running are marked `lost` with their partial records kept. Resuming them is v0.2, where agents support it. |
| NFR-SEC-1 | See `specs/security.md`: socket permissions, per-session MCP tokens, no secrets on disk, plugin permissions. |
| NFR-PORT-1 | macOS 13+ and mainstream Linux (glibc x86_64/arm64). Terminals: iTerm2, Terminal.app, Ghostty, WezTerm, Alacritty, Kitty, and inside tmux. |
| NFR-I18N-1 | Every user-facing string is localized (en, es); CI enforces key parity. |
| NFR-OSS-1 | Public-ready repo: README, CONTRIBUTING, CODE_OF_CONDUCT, SECURITY, templates; MIT license (ADR 0023). |
| NFR-MAINT-1 | The code is maintained by agents. Tests are the specification; every crate has a README stating its purpose and allowed dependencies. |

## 9. v0.1 scope: thin versions

The owner put all nine candidate features in the MVP (Q13.1). To keep this realistic, each ships in a thin but complete form:

| Feature (Q13.1) | In v0.1 | Deferred |
|---|---|---|
| Projects and tasks with kanban | Full (ORG-1…7) | External tracker sync (v0.2) |
| Agents in worktrees, watched live | 4 adapters. ACP where verified, PTY everywhere, headless parsers for Claude Code and Codex | Session resume after a daemon restart |
| Knowledge base + `CLAUDE.md`/`AGENTS.md` generation | Full CTX-1…13 except CTX-6b | graphify, Notion/Obsidian import |
| Harness profiles synced to Claude Code and Codex | Full. opencode/Cursor best-effort | Gemini target on by default; A/B harness comparison |
| HighSet MCP server | Full tool set in `specs/mcp-server.md` | — |
| Guided Spec → Plan → Implement | Flows, gates, checks, templates, PRs | Custom flow editor UI (flows are TOML files) |
| Full-text search | tantivy + fuzzy + filters | Semantic search |
| Costs and tokens | Capture, pricing, aggregates, budgets with alerts | Per-model optimization advice |
| Plugin system | Declarative packages, git install, permissions, Claude Code/Agent Skills import | Code plugins (out-of-process), registry |

Duration: **8 weeks** with parallel builder agents, confirmed by the owner (OQ-2, resolved 2026-10-03).

## 10. Success metrics (measured by dogfooding)

- Starting a task with an agent takes ≤ 3 keystrokes from the kanban and ≤ 30 s including context compilation.
- Zero manual context copy-paste in the owner's daily workflow after two weeks of use.
- ≥ 3 agents can run in parallel on different tasks with no git conflicts caused by HighSet.
- 100% of finished sessions have a transcript, summary, diff and usage record.
- All PERF budgets pass in CI and on the owner's machine.
- The owner stops opening agent CLIs directly for project work.

## 11. Risks

| Risk | Impact | Mitigation |
|---|---|---|
| Scope vs. timeline (all nine features in 8 weeks) | Late MVP | Thin versions (§9), parallel lanes. If the plan slips, the first candidates for v0.2 are plugins (D-008/D-009) and budget alerts (A-010); moving them needs the owner's approval |
| Agent CLIs change flags, config formats or log locations | Broken adapters | Isolated adapters; "verify facts" step; fixture-based tests; `highset doctor`; version detection |
| ACP support varies per agent | No structured view for some agents | PTY fallback for every agent; ACP adapters verified per agent |
| Owner cannot review code | Hidden quality issues | Tests as spec, reviewer agent, CI, behavior demos per milestone |
| Embedded terminal rendering cost | Breaks performance budgets | vt100 snapshots, coalesced output, render on dirty only, benchmarks |
| SQLite in synced folders corrupts | Data loss | Content/state split: only plain files may be synced (ADR-0022) |
| Secret leakage into transcripts or knowledge | Security incident | Redaction before persist, keychain-only secrets, tests with canary secrets |
