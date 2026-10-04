---
id: D-003
title: "Internal LLM tasks"
lane: platform
milestone: M4
size: M
status: todo
depends_on: [D-002, A-008]
requirements: [LLM-2, CTX-11]
open_questions: []
---

# D-003 · Internal LLM tasks

**Lane:** platform · **Milestone:** M4 · **Size:** M · **Depends on:** [D-002](D-002-llm-provider-layer.md), [A-008](A-008-session-recording.md)

## Goal

Summaries, memory extraction, tagging and spec drafts, in the background.

## Read first

- [`specs/llm.md#3-internal-tasks-llm-2`](../specs/llm.md#3-internal-tasks-llm-2)
- ADR [0019](../decisions/0019-internal-llm-tasks-through-a-provider-layer-with.md)
- ADR [0021](../decisions/0021-context-mentions-instead-of-auto-naming.md)
- PRD requirements: LLM-2, CTX-11

## Scope

- The four tasks in llm §3, with editable prompt templates and `prompt_version`.
- Background queue with visible status; failures never block; inputs and outputs redacted.
- Extraction output feeds the inbox (B-011 interface); `highset task spec T-n --draft`.

## Acceptance criteria

- [ ] Golden tests with recorded fixture responses for each task.
- [ ] With no provider configured, a session end still produces a placeholder summary.
- [ ] Extraction creates at most 5 proposals, each with evidence.
- [ ] `task spec --draft` writes a DRAFT spec into the task's spec dir.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
