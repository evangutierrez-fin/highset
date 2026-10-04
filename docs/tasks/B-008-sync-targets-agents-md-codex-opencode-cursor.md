---
id: B-008
title: "Sync targets: AGENTS.md, Codex, opencode, Cursor, Gemini"
lane: context
milestone: M3
size: M
status: todo
depends_on: [B-006, A-006]
requirements: [CTX-7, HRN-2]
open_questions: []
---

# B-008 · Sync targets: AGENTS.md, Codex, opencode, Cursor, Gemini

**Lane:** context · **Milestone:** M3 · **Size:** M · **Depends on:** [B-006](B-006-context-compiler.md), [A-006](A-006-codex-cli-adapter.md)

## Goal

Every other agent receives the same source of truth.

## Read first

- [`specs/context-compiler.md#32-targets-in-v01`](../specs/context-compiler.md#32-targets-in-v01)
- ADR [0009](../decisions/0009-highset-is-the-source-of-truth-for-harness-and.md)
- ADR [0018](../decisions/0018-follow-industry-standards-before-inventing.md)
- PRD requirements: CTX-7, HRN-2

## Scope

- `agents-md`, `codex` (using A-006's ADR strategy), `opencode`, `cursor` and `gemini` targets per context-compiler §3.2.
- Targets enabled per `sync.targets`; same ownership, dry-run and no-secrets rules as B-007.

## Acceptance criteria

- [ ] Idempotency, ownership and no-secrets tests for every target.
- [ ] The AGENTS.md block contains only unrestricted, non-local content (test).
- [ ] The user's Codex config and auth are untouched (test with a temp home).
- [ ] Manual check with a real Codex session documented.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
