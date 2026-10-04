---
id: B-006
title: "Context compiler"
lane: context
milestone: M3
size: L
status: todo
depends_on: [B-004, B-005]
requirements: [CTX-7, CTX-9, CTX-13]
open_questions: []
---

# B-006 · Context compiler

**Lane:** context · **Milestone:** M3 · **Size:** L · **Depends on:** [B-004](B-004-context-packs.md), [B-005](B-005-harness-profiles-and-mcp-server-registry.md)

## Goal

Compute exactly what each agent should know for each task, explainably.

## Read first

- [`specs/context-compiler.md#2-compilation`](../specs/context-compiler.md#2-compilation)
- [`specs/context-compiler.md#5-explain-ctx-13`](../specs/context-compiler.md#5-explain-ctx-13)
- ADR [0009](../decisions/0009-highset-is-the-source-of-truth-for-harness-and.md)
- ADR [0020](../decisions/0020-knowledge-scoping-levels-plus-audience.md)
- PRD requirements: CTX-7, CTX-9, CTX-13

## Scope

- The pipeline in context-compiler §2: layers, profile, sources, filtering, classification, budgets, provenance, hash.
- Cache by hash; `context.compile` and `context.explain`; CLI `highset context explain`.
- Integrate with `session.start` (replacing A-002's minimal bundle).

## Acceptance criteria

- [ ] Golden (insta) bundles for three fixture projects: software, research, and one with audience-restricted items.
- [ ] Identical inputs give identical bytes and hash; any input change gives a different hash.
- [ ] Budget overflow moves items to on-demand, with provenance entries (test).
- [ ] Warm compile ≤ 5 ms and cold compile ≤ 100 ms on the fixture project (bench).
- [ ] `context explain` shows included and excluded items with reasons and token estimates.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
