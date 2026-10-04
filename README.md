# HighSet

[![CI](https://github.com/evangutierrez-fin/highset/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/evangutierrez-fin/highset/actions/workflows/ci.yml)

**A command center for working with AI coding agents, from your terminal.**

HighSet organizes your agent work into workspaces, projects, tasks and sessions. It gives every agent the right context from a single source of truth, keeps their configuration in sync, and runs several agents in parallel without them stepping on each other. It also guides each piece of work through a lightweight spec-driven method.

It doesn't replace Claude Code, Codex CLI, opencode or Cursor CLI. It orchestrates them.

> **Status:** planning complete, implementation not started. See [`docs/ROADMAP.md`](docs/ROADMAP.md).
> Leer en español: [`docs/es/LEEME.md`](docs/es/LEEME.md).

## Planned for v0.1

- **Organization.** Workspaces → projects → tasks → sessions, a kanban board, and quick capture (`highset add "idea"`).
- **Agents in parallel.** Each task gets its own git worktree and branch. You watch agents live (ACP or an embedded terminal) and approve permission requests from one place.
- **Context.**
  - A knowledge base with global, workspace, project and private levels, plus per-agent visibility.
  - Context packs and `@`-mentions.
  - `CLAUDE.md` / `AGENTS.md` generated from one source.
  - A built-in MCP server agents can query.
- **Harness.** Agent profiles (instructions, model, MCP servers, skills, permissions, hooks) defined once and synced to Claude Code and Codex.
- **Method.** Spec → Plan → Implement → Verify with approval gates, templates, daily standups, ADRs and a changelog.
- **Also:**
  - Full-text and fuzzy search over everything.
  - Token and cost tracking with budgets.
  - Declarative plugins compatible with Claude Code plugins and Agent Skills.
- **Fast and local.** A single Rust binary with no telemetry. Secrets live in your OS keychain.

## Documentation

| Doc | Purpose |
|---|---|
| [`docs/PRD.md`](docs/PRD.md) | What HighSet does and why |
| [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) | How it is built |
| [`docs/ROADMAP.md`](docs/ROADMAP.md) | Phases and milestones |
| [`docs/specs/`](docs/specs/) | Module specifications |
| [`docs/decisions/`](docs/decisions/) | Architecture decision records |
| [`docs/tasks/`](docs/tasks/) | Build backlog |

## License

[MIT](LICENSE).

Repository: <https://github.com/evangutierrez-fin/highset>
