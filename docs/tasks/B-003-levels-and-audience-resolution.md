---
id: B-003
title: "Levels and audience resolution"
lane: context
milestone: M3
size: M
status: todo
depends_on: [B-001]
requirements: [CTX-2, CTX-3]
open_questions: []
---

# B-003 · Levels and audience resolution

**Lane:** context · **Milestone:** M3 · **Size:** M · **Depends on:** [B-001](B-001-knowledge-base-and-convention-files.md)

## Goal

Guarantee that private or targeted knowledge only reaches the agents it is meant for.

## Read first

- [`specs/knowledge.md#3-audience-who-can-see-an-item-answering-the-q53-note`](../specs/knowledge.md#3-audience-who-can-see-an-item-answering-the-q53-note)
- ADR [0020](../decisions/0020-knowledge-scoping-levels-plus-audience.md)
- PRD requirements: CTX-2, CTX-3

## Scope

- `visible_items(VisibilityCtx{workspace, project, task, profile, agent})` with reasons for every exclusion.
- Audience AND semantics; shadowing; filter for committed outputs (unrestricted and non-local only).
- Used by the compiler, MCP and mentions.

## Acceptance criteria

- [ ] Table-driven tests for every rule in knowledge §2–3.
- [ ] Property test: an item with `audience.agents = [X]` is never visible to an agent other than X; local items never appear in committed outputs.
- [ ] Exclusion reasons are returned for `context explain`.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
