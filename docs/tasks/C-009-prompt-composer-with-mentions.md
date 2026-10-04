---
id: C-009
title: "Prompt composer with mentions"
lane: tui
milestone: M3
size: S
status: todo
depends_on: [C-004, B-010]
requirements: [CTX-10]
open_questions: []
---

# C-009 · Prompt composer with mentions

**Lane:** tui · **Milestone:** M3 · **Size:** S · **Depends on:** [C-004](C-004-live-session-view.md), [B-010](B-010-context-mentions.md)

## Goal

Write prompts that pull in exactly the right context.

## Read first

- [`specs/tui.md#4-live-session-pane`](../specs/tui.md#4-live-session-pane)
- [`specs/context-compiler.md#4-mentions-ctx-10-adr-0021`](../specs/context-compiler.md#4-mentions-ctx-10-adr-0021)
- ADR [0021](../decisions/0021-context-mentions-instead-of-auto-naming.md)
- PRD requirements: CTX-10

## Scope

- Multi-line composer; `@` opens the fuzzy picker; chips show what will be attached and its size; preview; `ctrl+enter` sends.

## Acceptance criteria

- [ ] Picker latency ≤ 30 ms.
- [ ] The preview matches `mention.resolve`.
- [ ] Snapshots.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
