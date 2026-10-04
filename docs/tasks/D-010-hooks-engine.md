---
id: D-010
title: "Hooks engine"
lane: platform
milestone: M4
size: M
status: todo
depends_on: [A-002]
requirements: [HRN-6]
open_questions: []
---

# D-010 · Hooks engine

**Lane:** platform · **Milestone:** M4 · **Size:** M · **Depends on:** [A-002](A-002-agent-adapter-framework-and-session-supervisor.md)

## Goal

Run your own scripts at the right moments.

## Read first

- [`specs/harness.md#6-hooks`](../specs/harness.md#6-hooks)
- ADR [0009](../decisions/0009-highset-is-the-source-of-truth-for-harness-and.md)
- PRD requirements: HRN-6

## Scope

- `hooks.toml` at every level; events; environment and stdin JSON; timeouts; blocking `pre_commit` and `pre_merge`; attention on failure; `highset hook list|test`.

## Acceptance criteria

- [ ] Tests for every event; a failing `pre_merge` hook blocks the merge and shows its (redacted) stderr.
- [ ] Timeouts are enforced.
- [ ] Plugin hooks run only with approved permissions.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
