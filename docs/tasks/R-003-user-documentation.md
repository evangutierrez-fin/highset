---
id: R-003
title: "User documentation"
lane: release
milestone: M5
size: M
status: todo
depends_on: [D-005, C-010]
requirements: [NFR-OSS-1, UX-9]
open_questions: []
---

# R-003 · User documentation

**Lane:** release · **Milestone:** M5 · **Size:** M · **Depends on:** [D-005](D-005-methodology-execution-phases-gates-checks-finish.md), [C-010](C-010-i18n-completeness-and-ux-polish.md)

## Goal

Anyone can install, understand and extend HighSet, in English or Spanish.

## Read first

- [`ROADMAP.md#m5-v010--2026-12-01`](../ROADMAP.md#m5-v010--2026-12-01)
- ADR [0018](../decisions/0018-follow-industry-standards-before-inventing.md)
- PRD requirements: NFR-OSS-1, UX-9

## Scope

- `docs/user/` (en and es): install, 10-minute quickstart, concepts, agent setup per adapter, plugin authoring.
- Config reference generated from the schema (`xtask docs config`) and keymap reference generated from the registry.

## Acceptance criteria

- [ ] Generated references come from commands and CI fails on drift.
- [ ] The quickstart is followed on a clean user account (documented).
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
