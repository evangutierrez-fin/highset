---
id: A-002
title: "Agent adapter framework and session supervisor"
lane: agents
milestone: M2
size: L
status: todo
depends_on: [A-001, P0-006]
requirements: [AGT-1, AGT-4, AGT-5, AGT-8]
open_questions: []
---

# A-002 · Agent adapter framework and session supervisor

**Lane:** agents · **Milestone:** M2 · **Size:** L · **Depends on:** [A-001](A-001-git-service-worktrees-branches-commits-diffs.md), [P0-006](P0-006-test-harness-and-fake-agent.md)

## Goal

The machinery to run any agent as a session on a task, observable and persistent.

## Read first

- [`specs/agents.md#1-adapter-trait`](../specs/agents.md#1-adapter-trait)
- [`specs/agents.md#2-normalized-sessionevent`](../specs/agents.md#2-normalized-sessionevent)
- [`specs/agents.md#4-session-supervisor-daemon`](../specs/agents.md#4-session-supervisor-daemon)
- ADR [0005](../decisions/0005-agent-integration-acp-first-embedded-pty.md)
- PRD requirements: AGT-1, AGT-4, AGT-5, AGT-8

## Scope

- `AgentAdapter`, `Detection`, `LaunchSpec`, `RunningSession`, `SessionEvent` in `highset-agents` (agents §1–2).
- Adapter registry with the `Fake` adapter wrapping `tools/fake-agent`.
- Daemon `services/sessions`: `session.start|stop|list|get`; per-session env (`HIGHSET_SESSION_ID`, `HIGHSET_SESSION_TOKEN`, `HIGHSET_SOCKET`, `HIGHSET_TASK`); `session.toml` persistence; event publishing; `max_per_task`; `lost` marking on daemon start.
- Until B-006 exists, build a minimal bundle containing only the task brief.
- CLI: `highset agent list`, `highset session start|list|show|stop|log`.

## Acceptance criteria

- [ ] With the fake adapter, a session starts in the task worktree, emits events and ends; `session.toml` exists with the right fields.
- [ ] Four concurrent fake sessions on different tasks run correctly (integration test).
- [ ] Restarting the daemon marks running sessions as `lost`.
- [ ] A second session on the same task is refused when `max_per_task = 1`.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
