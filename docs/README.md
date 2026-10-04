# HighSet docs

## Reading order for builder agents

1. [`../AGENTS.md`](../AGENTS.md): how to work in this repo (workflow, DoD, conventions).
2. [`PRD.md`](PRD.md): product requirements. Every requirement has an ID (`ORG-1`, `CTX-7`…).
3. [`ARCHITECTURE.md`](ARCHITECTURE.md): crates, process model, data model, key flows.
4. [`ROADMAP.md`](ROADMAP.md): phases, lanes, milestones, owner demos.
5. [`specs/`](specs/): read the specs your task links to.
6. [`tasks/README.md`](tasks/README.md): the board. Pick your task there.
7. [`decisions/`](decisions/): ADRs. They are binding.
8. [`OPEN-QUESTIONS.md`](OPEN-QUESTIONS.md): what is still waiting on the owner.

## Specs

| Spec | Covers |
|---|---|
| [core-domain](specs/core-domain.md) | Entities, IDs, task status machine, events, ContextBundle IR |
| [storage](specs/storage.md) | File layout, file formats, SQLite cache, watcher, backups |
| [daemon-protocol](specs/daemon-protocol.md) | Process model, socket, JSON-RPC methods and events |
| [cli](specs/cli.md) | Command tree |
| [agents](specs/agents.md) | Adapter trait, ACP client, PTY, headless, per-agent notes, sessions, attention |
| [git](specs/git.md) | Worktrees, branches, checkpoint commits, diffs, merge, PRs |
| [knowledge](specs/knowledge.md) | Knowledge items, convention files, levels and audience, ingestion, memory, staleness |
| [context-compiler](specs/context-compiler.md) | Packs, bundle compilation, sync targets, mentions, explain |
| [harness](specs/harness.md) | Profiles, built-in roles, layering, permissions, hooks, MCP server registry |
| [mcp-server](specs/mcp-server.md) | Tools and resources HighSet exposes to agents |
| [methodology](specs/methodology.md) | Flows, gates, checks, templates, rituals, logs |
| [search](specs/search.md) | Full-text and fuzzy search |
| [costs](specs/costs.md) | Usage capture, pricing, budgets |
| [plugins](specs/plugins.md) | Declarative packages, install, permissions, Claude Code compatibility |
| [llm](specs/llm.md) | Provider layer and internal LLM tasks |
| [tui](specs/tui.md) | Layout, views, keymaps, palette, rendering, i18n |
| [security](specs/security.md) | Threat model, secrets, redaction, permissions |
| [performance](specs/performance.md) | Budgets and how they are measured |

## For the owner (Spanish)

- [`es/LEEME.md`](es/LEEME.md): what's here, how to start the build, how to run lanes in parallel.
- [`es/revision-de-fases.md`](es/revision-de-fases.md): how to review each milestone without reading code.
- [`inputs/questionnaire-2026-10-03.md`](inputs/questionnaire-2026-10-03.md): your original answers.
