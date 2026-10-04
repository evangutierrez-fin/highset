# AGENTS.md: HighSet

Instructions for any AI coding agent (Claude Code, Codex, opencode, Cursor…) working on this repository.
Claude Code also reads `CLAUDE.md`, which imports this file.

## What you are building

HighSet is a keyboard-first, local-first command center for working with AI coding agents from the terminal.
It **orchestrates** existing agents (Claude Code, Codex CLI, opencode, Cursor CLI); it is not an agent itself.
It:

- organizes work as workspaces → projects → tasks → sessions;
- compiles the right context for each agent from one source of truth;
- syncs harness configuration (instructions, model, MCP servers, skills, permissions, hooks) to every agent;
- runs agents in parallel, each in its own git worktree;
- guides the work with an adaptive spec-driven methodology.

It ships as a single Rust binary, `highset`: a long-running daemon plus thin clients (TUI, CLI, and an MCP stdio server that agents launch).

## Read before you write code

1. `docs/README.md`: map of the docs.
2. `docs/PRD.md`: what and why. Requirement IDs (e.g. `CTX-7`) are referenced everywhere.
3. `docs/ARCHITECTURE.md`: crates, process model, data flow, dependency rules.
4. `docs/ROADMAP.md`: phases, lanes, milestones.
5. The spec(s) your task links to in `docs/specs/`.
6. Your task file in `docs/tasks/`.
7. Relevant ADRs in `docs/decisions/`. **ADRs are binding.** To deviate, follow "Changing a decision" below.

## The owner

- The owner **does not read Rust** and will not review code. They review *behavior* at the end of each milestone.
- So tests are the real specification. Every acceptance criterion must map to an automated test or to an explicit, documented manual check.
- Prefer boring, explicit code over clever code: future agents maintain this codebase, not the owner.
- Talk to the owner in **Spanish** (questions, status, milestone reports). Write code, comments, commits and technical docs in **English**.
- Some decisions belong to the owner. Log them in `docs/OPEN-QUESTIONS.md`, keep working on whatever is not blocked, and reference the question ID in the task.

## Task workflow

Every task follows the same adaptive spec-driven loop that HighSet itself will later provide.

1. **Pick.** Open `docs/tasks/README.md`. Take the first task in your lane with `status: todo` whose `depends_on` tasks are all `done`. If there is none, stop and report.
2. **Claim.** Create a branch `task/<ID>-<slug>` (in its own git worktree when other agents are active). Set `status: in-progress` in the task file and on the board, and commit.
3. **Plan.** For size M/L tasks, and for anything that touches a contract (`highset-core`, `highset-protocol`, file formats), write a short plan first. It lists the tests to write, the files and crates touched, and the risks. Claude Code uses the `planner` subagent for this.
4. **Verify external facts.** Tasks that depend on a third-party CLI, protocol, file format or crate API need a check of the current official docs or source before you code. Examples: agent CLI flags, ACP, MCP, Claude Code/Codex/opencode/Cursor config formats, Spec Kit layout. Record what you confirmed in the spec's **"Verified facts"** section, with date and source URL. The planning docs were written on 2026-10-03 and may be stale.
5. **Implement** in small commits, with tests written alongside the code (tests first where practical).
6. **Verify** against the Definition of Done below.
7. **Review.** Get an independent review: Claude Code uses the `reviewer` subagent, other agents do a self-review against the DoD and the specs. Fix all blocking findings.
8. **Finish:**
   - Fill the task's "Verification log".
   - Set `status: done`, or `review` when a manual check by the owner is still needed.
   - Update the board.
   - Add a `CHANGELOG.md` entry under *Unreleased* if the change is user-visible.
   - Update specs if the behavior differs from them.
   - Rebase on `main`, rerun the DoD, merge, and remove your worktree.

### Definition of Done

- [ ] All acceptance criteria met. Each one maps to a test or a documented manual check in the task's verification log.
- [ ] `cargo fmt --all -- --check` is clean.
- [ ] `cargo clippy --workspace --all-targets --all-features -- -D warnings` is clean.
- [ ] `cargo test --workspace` is green, and CI is green on macOS and Linux.
- [ ] No new `unwrap`/`expect`/`panic!` in non-test code, unless a comment explains why it cannot fail.
- [ ] Public items have doc comments. The crate `README.md` is updated if the crate's responsibilities changed.
- [ ] User-facing strings are Fluent keys with **en** and **es** translations.
- [ ] Touched code paths respect the budgets in `docs/specs/performance.md`.
- [ ] No secrets in code, fixtures or logs.
- [ ] Specs and ADRs reflect what was built.

## Parallel work rules

- **Lanes:**
  - `infra` and `core` (Phases 0–1) are sequential and done by one agent.
  - After milestone M1, four lanes run in parallel: `agents`, `context`, `tui`, `platform`.
  - `release` comes last.
- One agent per lane at a time, and one task per branch/worktree.
- **Contracts freeze at M1.** `highset-core` types, `highset-protocol` methods/events, and file formats in `docs/specs/storage.md` are contracts.
  - Additive changes are allowed with a note in the spec: a new optional field, a new method, a new event.
  - Breaking changes need an ADR plus a dedicated small change merged to `main` first. Other lanes then rebase.
- Daemon code for each lane lives in `crates/highset-daemon/src/services/<area>/` and is registered with one line in the service registry. This keeps merge conflicts small.
- Never edit another lane's crate in the same branch. If you need something from another lane, add a note to its task or to `docs/OPEN-QUESTIONS.md`.

## Commands

| Purpose | Command |
|---|---|
| Format | `cargo fmt --all` |
| Lint | `cargo clippy --workspace --all-targets --all-features -- -D warnings` |
| Test | `cargo test --workspace` |
| Supply chain | `cargo deny check` |
| Dependency rules | `cargo run -p xtask -- check-deps` |
| i18n parity | `cargo run -p xtask -- check-i18n` |
| Run the app | `cargo run -p highset-cli -- <args>` (binary name: `highset`) |
| Run the daemon in the foreground | `cargo run -p highset-cli -- daemon run` |

`xtask` is a small helper crate created in P0-001/P0-002. Keep it the single place for repo automation.

## Coding conventions

- **Toolchain.** Rust stable, pinned in `rust-toolchain.toml`, edition 2024.
- **Errors.** Use `thiserror` enums per library crate and `anyhow` only in `highset-cli`'s `main`. Errors carry context (path, id, command). They are mapped to protocol error codes in one place (`highset-protocol::error`). Messages say what happened and how to fix it.
- **No panics** on paths reachable from user input or agent output.
- **Async.** Use `tokio` and never block the runtime:
  - SQLite access goes through the store actor.
  - Blocking IO or heavy CPU goes to `spawn_blocking` or a dedicated thread.
- **Logging.** Use `tracing` with structured fields. No `println!` outside CLI output code. Never log secrets, and never log full prompts or transcripts above `debug`.
- **Unsafe.** Every crate has `#![forbid(unsafe_code)]`. Exceptions need an ADR.
- **Subprocesses.** Never use `sh -c` with interpolated input; use `std::process::Command` / `tokio::process::Command` with explicit args.
- **Paths.** Never hardcode `~/.highset` or other locations; use `highset_core::paths`.
- **User-facing text.** All strings are Fluent keys in `highset-i18n` (en + es).
- **Dependencies.** Prefer well-maintained crates. Every new dependency must pass `cargo deny`. Justify heavy ones (compile time, binary size) in the task's verification log.
- **Tests:**
  - Unit tests live next to the code; integration tests live in `tests/`.
  - TUI tests use `insta` snapshots with ratatui's `TestBackend`.
  - Agent behavior is tested with `tools/fake-agent`.
  - **Never call real agents or real model APIs in CI.** Real-agent checks are documented manual tests.

## Performance is a feature

The owner's single deal-breaker is a slow or heavy app (Q13.9). The budgets in `docs/specs/performance.md` are release-blocking. Here are the headline numbers:

- TUI first frame in ≤ 150 ms.
- Keypress to frame in ≤ 16 ms (p95).
- One-shot CLI commands in ≤ 60 ms.
- Daemon idle CPU ≈ 0 and idle RSS ≤ 60 MB.

Design for them from the first line: no polling loops, render only when state changes, cache instead of recomputing, and keep allocations in hot paths in check.

## Security rules

- Secrets live in the OS keychain. They are referenced as `secret:<name>` and resolved only inside the daemon at launch time, then injected as environment variables. They are never written to files.
- Redaction runs before anything is persisted: transcripts, knowledge, memory, MCP outputs, logs.
- The daemon socket is `0600` inside the user's state dir. MCP sessions authenticate with a per-session token.
- No network access except for features the user explicitly invokes: URL ingest, LLM calls, plugin install, `gh`. No telemetry, ever.

## Git conventions

- Branches: `task/<ID>-<slug>`, e.g. `task/P1-003-sqlite-cache`.
- Commits follow Conventional Commits (`feat(store): …`, `fix(tui): …`), with a footer `Refs: <task-id>`.
- Never force-push `main`. Never commit secrets, local state or build artifacts.

## Changing a decision

1. Write a new ADR in `docs/decisions/` that supersedes the old one, with status `Proposed`.
2. If the change alters owner-visible behavior, scope or timeline, add it to `docs/OPEN-QUESTIONS.md` and ask the owner, in Spanish.
3. Otherwise mark it `Accepted`, update the affected specs, and continue.

## End of a milestone

When the last task of a milestone (see `docs/ROADMAP.md`) is done:

1. Write the milestone report **in Spanish** at `docs/es/reportes/M<N>.md`, following `docs/es/revision-de-fases.md`. It covers what works, exact demo steps the owner can copy-paste, known issues, decisions taken, and what comes next.
2. Stop and wait for the owner's review.

## Repository map

```
AGENTS.md / CLAUDE.md      instructions for builder agents
docs/
  README.md                docs map and reading order
  PRD.md                   product requirements (IDs like ORG-3, CTX-7)
  ARCHITECTURE.md          system design, crates, diagrams
  ROADMAP.md               phases, lanes, milestones
  OPEN-QUESTIONS.md        decisions waiting for the owner
  decisions/               ADRs (MADR format)
  specs/                   one spec per module
  tasks/                   backlog: README.md board + one file per task
  inputs/                  original questionnaire answers (source of truth for intent)
  es/                      Spanish docs for the owner (guide, phase reviews, reports)
crates/                    Rust workspace (created by P0-001)
tools/fake-agent/          test double for agents (P0-006)
xtask/                     repo automation (P0-001)
.claude/                   builder subagents and skills
```
