---
id: D-005
title: "Methodology execution: phases, gates, checks, finish"
lane: platform
milestone: M4
size: L
status: todo
depends_on: [D-004, A-002, A-001, B-006]
requirements: [MTH-1, MTH-2, MTH-3, MTH-8, INT-1]
open_questions: []
---

# D-005 · Methodology execution: phases, gates, checks, finish

**Lane:** platform · **Milestone:** M4 · **Size:** L · **Depends on:** [D-004](D-004-methodology-engine-and-templates.md), [A-002](A-002-agent-adapter-framework-and-session-supervisor.md), [A-001](A-001-git-service-worktrees-branches-commits-diffs.md), [B-006](B-006-context-compiler.md)

## Goal

Take a task from idea to merged PR with the right agent at each phase and human gates where they matter.

## Read first

- [`specs/methodology.md#3-gates-mth-2`](../specs/methodology.md#3-gates-mth-2)
- [`specs/methodology.md#4-checks-mth-3`](../specs/methodology.md#4-checks-mth-3)
- [`specs/methodology.md#5-execution-commands`](../specs/methodology.md#5-execution-commands)
- [`specs/git.md#4-finishing-a-task`](../specs/git.md#4-finishing-a-task)
- ADR [0013](../decisions/0013-methodology-adaptive-spec-driven-development.md)
- ADR [0006](../decisions/0006-one-git-worktree-and-branch-per-task.md)
- PRD requirements: MTH-1, MTH-2, MTH-3, MTH-8, INT-1

## Scope

- `task start|advance|done`, `gate approve|reject`; checks runner; reviewer session; checkpoint commits.
- Finish via PR (`gh`) or local merge; staleness trigger (B-012); changelog trigger (D-006 interface); worktree cleanup.
- Make the TUI's start key call `task.start`.

## Acceptance criteria

- [ ] End-to-end test with the fake agent through the feature flow: spec → gate → plan → gate → implement → checks → review agent → approve_diff → local merge.
- [ ] Failing checks block `approve_diff` unless overridden with a logged reason.
- [ ] A rejected gate returns the task to the previous phase, and the reason appears in the next session's prompt.
- [ ] Without `gh`, `--merge pr` prints the exact commands and leaves the task in review.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
