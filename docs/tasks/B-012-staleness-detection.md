---
id: B-012
title: "Staleness detection"
lane: context
milestone: M4
size: S
status: todo
depends_on: [B-001, A-001]
requirements: [CTX-12]
open_questions: []
---

# B-012 · Staleness detection

**Lane:** context · **Milestone:** M4 · **Size:** S · **Depends on:** [B-001](B-001-knowledge-base-and-convention-files.md), [A-001](A-001-git-service-worktrees-branches-commits-diffs.md)

## Goal

Docs don't silently go out of date after a task changes the code they describe.

## Read first

- [`specs/knowledge.md#7-staleness-ctx-12`](../specs/knowledge.md#7-staleness-ctx-12)
- ADR [0020](../decisions/0020-knowledge-scoping-levels-plus-audience.md)
- PRD requirements: CTX-12

## Scope

- `covers` globs plus the body-mention heuristic; on `task.done`, create `doc_update` proposals with the diff excerpt; precision first.

## Acceptance criteria

- [ ] Fixture: a change to a covered file creates exactly one proposal with a diff excerpt; an unrelated change creates none.
- [ ] Heuristic false-positive cases are documented in tests.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
