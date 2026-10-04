---
id: R-001
title: "Performance verification"
lane: release
milestone: M5
size: M
status: todo
depends_on: [C-010, D-005, D-007, A-010]
requirements: [PERF-1, PERF-2, PERF-3, PERF-4, PERF-5, PERF-6, PERF-7, PERF-8, PERF-9, PERF-10]
open_questions: []
---

# R-001 · Performance verification

**Lane:** release · **Milestone:** M5 · **Size:** M · **Depends on:** [C-010](C-010-i18n-completeness-and-ux-polish.md), [D-005](D-005-methodology-execution-phases-gates-checks-finish.md), [D-007](D-007-search.md), [A-010](A-010-costs-usage-and-budgets.md)

## Goal

Prove the app is as fast and light as promised.

## Read first

- [`specs/performance.md`](../specs/performance.md)
- ADR [0017](../decisions/0017-performance-budgets-are-release-blocking.md)
- PRD requirements: PERF-1, PERF-2, PERF-3, PERF-4, PERF-5, PERF-6, PERF-7, PERF-8, PERF-9, PERF-10

## Scope

- `xtask perf gen|tui-start|cli|idle|reindex`, criterion benches, the CI `perf-smoke` job with 2× thresholds.
- Report in `docs/perf/v0.1.0.md`.

## Acceptance criteria

- [ ] All PERF-1…10 are measured on the reference machines and met, or each miss has a superseding ADR and a v0.2 task approved by the owner.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
