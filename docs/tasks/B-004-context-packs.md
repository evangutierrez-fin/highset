---
id: B-004
title: "Context packs"
lane: context
milestone: M3
size: M
status: todo
depends_on: [B-003]
requirements: [CTX-5]
open_questions: []
---

# B-004 · Context packs

**Lane:** context · **Milestone:** M3 · **Size:** M · **Depends on:** [B-003](B-003-levels-and-audience-resolution.md)

## Goal

Reusable bundles of context that make starting new work fast.

## Read first

- [`specs/context-compiler.md#1-context-packs-ctx-5`](../specs/context-compiler.md#1-context-packs-ctx-5)
- ADR [0009](../decisions/0009-highset-is-the-source-of-truth-for-harness-and.md)
- PRD requirements: CTX-5

## Scope

- `pack.toml` format, activation scopes (global, workspace, project, profile, task), composition and conflict rules.
- CLI `highset pack list|new|show|enable|disable`.
- Starter packs `software-core`, `research`, `data-analysis`, `automation` with concise, genuinely useful content.

## Acceptance criteria

- [ ] Composition tests: deduplication, conflict messages, scope precedence.
- [ ] Starter packs validate, and the reviewer subagent approves their content for usefulness and brevity.
- [ ] Enabling a pack on a task affects only that task's compilation (test).
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
