---
id: P1-001
title: "Domain model and ContextBundle IR"
lane: core
milestone: M1
size: M
status: todo
depends_on: [P0-004]
requirements: [ORG-3, ORG-4, ORG-5, AGT-7, CTX-2, CTX-3]
open_questions: []
---

# P1-001 · Domain model and ContextBundle IR

**Lane:** core · **Milestone:** M1 · **Size:** M · **Depends on:** [P0-004](P0-004-paths-and-layered-configuration.md)

## Goal

Define the contract types every crate uses, including the task status machine and the agent-neutral ContextBundle.

## Read first

- [`specs/core-domain.md`](../specs/core-domain.md)
- ADR [0002](../decisions/0002-rust-single-binary-cargo-workspace-of-focused.md)
- ADR [0013](../decisions/0013-methodology-adaptive-spec-driven-development.md)
- ADR [0020](../decisions/0020-knowledge-scoping-levels-plus-audience.md)
- PRD requirements: ORG-3, ORG-4, ORG-5, AGT-7, CTX-2, CTX-3

## Scope

- IDs (core-domain §1) with prefix validation, Display/FromStr, serde as strings; `TaskNumber` (`T-0001`) and global-inbox numbers (`I-0001`).
- Entities from core-domain §2, with `extra` bags for unknown fields where noted.
- Flow types and `validate_transition(flow, from, to, actor, gates, strictness)` per core-domain §3. Built-in `feature` and `fix` flows as embedded TOML (they move to `highset-method` in D-004; the loader stays in core).
- `Event` enum and topics (core-domain §4).
- `ContextBundle` and related types (core-domain §5); `PermissionSet` parser and matcher for the neutral syntax (harness §4).
- Doc comments on every public item.

## Acceptance criteria

- [ ] Property tests: ID round trip; wrong prefix rejected.
- [ ] Table-driven tests for every transition in both flows × actor (human/agent) × strictness (guide/strict/info), including gate skips with a reason.
- [ ] Agents can never set `done` or `canceled` or approve gates (tests).
- [ ] PermissionSet tests: deny beats allow; globs; command patterns; `<verify>` expansion hook.
- [ ] `highset-core` depends on no async runtime, database or network crate (enforced by `xtask check-deps`).
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
