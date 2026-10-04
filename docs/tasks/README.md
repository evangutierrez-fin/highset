# Build backlog (board)

This board is the single place to see build progress. **Update it in the same commit as the task file** whenever a task's status changes.

- **Statuses:** `todo` → `in-progress` → `review` (waiting for an owner manual check) → `done`; `blocked` (say why in the task file).
- **Sizes:** S ≈ half a day of agent work, M ≈ one day, L ≈ two to three days.
- **How to pick:** the first `todo` task in your lane whose dependencies are all `done` (see `AGENTS.md` → Task workflow).
- **Lanes:** `infra` and `core` run sequentially (Phases 0–1). After M1, `agents`, `context`, `tui` and `platform` run in parallel. `release` comes last.

Total: 60 tasks.

## M1 · Core alive

| ID | Task | Lane | Size | Depends on | Status |
|---|---|---|---|---|---|
| [P0-001](P0-001-bootstrap-the-cargo-workspace.md) | Bootstrap the Cargo workspace | infra | S | — | done |
| [P0-002](P0-002-ci-pipeline.md) | CI pipeline | infra | S | P0-001 | todo |
| [P0-003](P0-003-open-source-repository-scaffolding.md) | Open-source repository scaffolding | infra | S | P0-001 | todo |
| [P0-004](P0-004-paths-and-layered-configuration.md) | Paths and layered configuration | infra | M | P0-001 | todo |
| [P0-005](P0-005-errors-logging-and-i18n-scaffold.md) | Errors, logging and i18n scaffold | infra | M | P0-001 | todo |
| [P0-006](P0-006-test-harness-and-fake-agent.md) | Test harness and fake agent | infra | M | P0-001 | todo |
| [P1-001](P1-001-domain-model-and-contextbundle-ir.md) | Domain model and ContextBundle IR | core | M | P0-004 | todo |
| [P1-002](P1-002-content-store-markdown-and-toml-files.md) | Content store: Markdown and TOML files | core | M | P1-001 | todo |
| [P1-003](P1-003-sqlite-cache-migrations-and-file-watcher.md) | SQLite cache, migrations and file watcher | core | M | P1-002 | todo |
| [P1-004](P1-004-protocol-crate-v1.md) | Protocol crate v1 | core | M | P1-001 | todo |
| [P1-005](P1-005-daemon-skeleton-and-client-autostart.md) | Daemon skeleton and client autostart | core | M | P1-003, P1-004 | todo |
| [P1-006](P1-006-workspaces-and-projects.md) | Workspaces and projects | core | M | P1-005 | todo |
| [P1-007](P1-007-tasks-quick-capture-and-ui-bootstrap.md) | Tasks, quick capture and UI bootstrap | core | M | P1-006 | todo |

## M2 · Agents live

| ID | Task | Lane | Size | Depends on | Status |
|---|---|---|---|---|---|
| [A-001](A-001-git-service-worktrees-branches-commits-diffs.md) | Git service: worktrees, branches, commits, diffs | agents | M | P1-005 | todo |
| [A-002](A-002-agent-adapter-framework-and-session-supervisor.md) | Agent adapter framework and session supervisor | agents | L | A-001, P0-006 | todo |
| [A-003](A-003-pty-runner-and-terminal-streaming.md) | PTY runner and terminal streaming | agents | L | A-002 | todo |
| [A-004](A-004-acp-client.md) | ACP client | agents | L | A-002 | todo |
| [A-005](A-005-claude-code-adapter.md) | Claude Code adapter | agents | L | A-003, A-004, A-009 | todo |
| [A-008](A-008-session-recording.md) | Session recording | agents | M | A-002, D-001 | todo |
| [A-009](A-009-attention-notifications-and-hook-signals.md) | Attention, notifications and hook signals | agents | M | A-002 | todo |
| [C-001](C-001-tui-shell-state-architecture-and-layout.md) | TUI shell, state architecture and layout | tui | L | P1-005, P0-005 | todo |
| [C-002](C-002-keymap-help-bar-and-command-palette.md) | Keymap, help bar and command palette | tui | M | C-001 | todo |
| [C-003](C-003-sidebar-task-list-kanban-and-task-detail.md) | Sidebar, task list, kanban and task detail | tui | M | C-002, P1-007 | todo |
| [C-004](C-004-live-session-view.md) | Live session view | tui | L | C-001, A-003, A-004 | todo |
| [D-001](D-001-secrets-and-redaction.md) | Secrets and redaction | platform | M | P1-005 | todo |

## M3 · Context and harness

| ID | Task | Lane | Size | Depends on | Status |
|---|---|---|---|---|---|
| [A-006](A-006-codex-cli-adapter.md) | Codex CLI adapter | agents | L | A-003, A-004, A-009 | todo |
| [A-007](A-007-opencode-and-cursor-cli-adapters.md) | opencode and Cursor CLI adapters | agents | M | A-003, A-004, A-009 | todo |
| [B-001](B-001-knowledge-base-and-convention-files.md) | Knowledge base and convention files | context | M | P1-003 | todo |
| [B-002](B-002-knowledge-ingestion-pipeline.md) | Knowledge ingestion pipeline | context | M | B-001 | todo |
| [B-003](B-003-levels-and-audience-resolution.md) | Levels and audience resolution | context | M | B-001 | todo |
| [B-004](B-004-context-packs.md) | Context packs | context | M | B-003 | todo |
| [B-005](B-005-harness-profiles-and-mcp-server-registry.md) | Harness profiles and MCP server registry | context | L | B-003 | todo |
| [B-006](B-006-context-compiler.md) | Context compiler | context | L | B-004, B-005 | todo |
| [B-007](B-007-sync-target-claude-code.md) | Sync target: Claude Code | context | M | B-006 | todo |
| [B-008](B-008-sync-targets-agents-md-codex-opencode-cursor.md) | Sync targets: AGENTS.md, Codex, opencode, Cursor, Gemini | context | M | B-006, A-006 | todo |
| [B-009](B-009-highset-mcp-server.md) | HighSet MCP server | context | L | B-006, P1-007 | todo |
| [B-010](B-010-context-mentions.md) | Context mentions (@) | context | M | B-006 | todo |
| [C-007](C-007-knowledge-packs-and-profiles-views.md) | Knowledge, packs and profiles views | tui | M | C-002, B-004, B-005 | todo |
| [C-009](C-009-prompt-composer-with-mentions.md) | Prompt composer with mentions | tui | S | C-004, B-010 | todo |
| [D-002](D-002-llm-provider-layer.md) | LLM provider layer | platform | M | D-001 | todo |
| [D-004](D-004-methodology-engine-and-templates.md) | Methodology engine and templates | platform | M | P1-007 | todo |
| [D-007](D-007-search.md) | Search | platform | M | P1-003 | todo |

## M4 · Method and platform

| ID | Task | Lane | Size | Depends on | Status |
|---|---|---|---|---|---|
| [A-010](A-010-costs-usage-and-budgets.md) | Costs, usage and budgets | agents | M | A-008 | todo |
| [B-011](B-011-inbox-memory-knowledge-and-decision-proposals.md) | Inbox: memory, knowledge and decision proposals | context | M | B-009 | todo |
| [B-012](B-012-staleness-detection.md) | Staleness detection | context | S | B-001, A-001 | todo |
| [C-005](C-005-diff-viewer-and-gate-actions.md) | Diff viewer and gate actions | tui | M | C-001, A-001 | todo |
| [C-006](C-006-dashboard-timeline-and-status-bar.md) | Dashboard, timeline and status bar | tui | M | C-003, A-009 | todo |
| [C-008](C-008-inbox-and-plugin-manager-views.md) | Inbox and plugin manager views | tui | M | C-002, B-011, D-008 | todo |
| [C-010](C-010-i18n-completeness-and-ux-polish.md) | i18n completeness and UX polish | tui | S | C-001, C-002, C-003, C-004, C-005, C-006, C-007, C-008, C-009 | todo |
| [D-003](D-003-internal-llm-tasks.md) | Internal LLM tasks | platform | M | D-002, A-008 | todo |
| [D-005](D-005-methodology-execution-phases-gates-checks-finish.md) | Methodology execution: phases, gates, checks, finish | platform | L | D-004, A-002, A-001, B-006 | todo |
| [D-006](D-006-logs-and-rituals.md) | Logs and rituals | platform | M | D-004, A-008 | todo |
| [D-008](D-008-plugins-v1-declarative-packages.md) | Plugins v1: declarative packages | platform | L | B-004, B-005 | todo |
| [D-009](D-009-claude-code-plugins-and-agent-skills-import.md) | Claude Code plugins and Agent Skills import | platform | M | D-008 | todo |
| [D-010](D-010-hooks-engine.md) | Hooks engine | platform | M | A-002 | todo |
| [D-011](D-011-backups-and-doctor.md) | Backups and doctor | platform | S | P1-005 | todo |

## M5 · v0.1.0

| ID | Task | Lane | Size | Depends on | Status |
|---|---|---|---|---|---|
| [R-001](R-001-performance-verification.md) | Performance verification | release | M | C-010, D-005, D-007, A-010 | todo |
| [R-002](R-002-packaging-and-release-pipeline.md) | Packaging and release pipeline | release | M | P0-002, R-001 | todo |
| [R-003](R-003-user-documentation.md) | User documentation | release | M | D-005, C-010 | todo |
| [R-004](R-004-dogfooding-highset-builds-highset.md) | Dogfooding: HighSet builds HighSet | release | S | D-005 | todo |

## Order within each lane

- **infra:** P0-001 → P0-002 → P0-003 → P0-004 → P0-005 → P0-006
- **core:** P1-001 → P1-002 → P1-003 → P1-004 → P1-005 → P1-006 → P1-007
- **agents:** A-001 → A-002 → A-003 → A-004 → A-005 → A-006 → A-007 → A-008 → A-009 → A-010
- **context:** B-001 → B-002 → B-003 → B-004 → B-005 → B-006 → B-007 → B-008 → B-009 → B-010 → B-011 → B-012
- **tui:** C-001 → C-002 → C-003 → C-004 → C-005 → C-006 → C-007 → C-008 → C-009 → C-010
- **platform:** D-001 → D-002 → D-003 → D-004 → D-005 → D-006 → D-007 → D-008 → D-009 → D-010 → D-011
- **release:** R-001 → R-002 → R-003 → R-004
