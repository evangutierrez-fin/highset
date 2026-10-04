---
id: B-010
title: "Context mentions (@)"
lane: context
milestone: M3
size: M
status: todo
depends_on: [B-006]
requirements: [CTX-10]
open_questions: []
---

# B-010 · Context mentions (@)

**Lane:** context · **Milestone:** M3 · **Size:** M · **Depends on:** [B-006](B-006-context-compiler.md)

## Goal

Reference any context in a prompt with `@`, like Cursor (the owner's Q2.4 note).

## Read first

- [`specs/context-compiler.md#4-mentions-ctx-10-adr-0021`](../specs/context-compiler.md#4-mentions-ctx-10-adr-0021)
- ADR [0021](../decisions/0021-context-mentions-instead-of-auto-naming.md)
- PRD requirements: CTX-10

## Scope

- Grammar, `mention.complete`, `mention.resolve`, size budgets and explicit truncation markers.
- Audience-aware resolution; ACP resource blocks vs. inline sections; `--prompt` support in the CLI.

## Acceptance criteria

- [ ] Parser tests for every kind, escaping, and ignoring emails and code spans.
- [ ] Resolution respects audience and levels.
- [ ] Oversized items are truncated with an explicit marker and an MCP pointer (test).
- [ ] `mention.complete` ≤ 30 ms on the 50k-candidate dataset.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
