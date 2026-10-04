# 0017. Performance budgets are release-blocking

- Status: Accepted
- Date: 2026-10-03
- Source: questionnaire Q13.7, Q13.9 (`docs/inputs/questionnaire-2026-10-03.md`)

## Context

The owner ranked performance 6th among goals (Q13.7) but named slowness and heavy compute as the one thing that would make them hate the app (Q13.9).

## Decision

Treat the budgets in `specs/performance.md` (PERF-1…10) as hard constraints: CI runs smoke checks with 2× thresholds; the full measurement on reference machines blocks releases. Design rules (no polling, render on dirty, caches, thin TUI) apply from the first task.

## Consequences

+ Prevents the death-by-a-thousand-cuts slowdown typical of feature-rich TUIs.
- Some features need extra engineering (PTY rendering, indexing) and may be deferred if they cannot meet budgets.

## Alternatives considered

Optimize later (historically never happens). Budgets as guidance only (does not protect the owner's deal-breaker).
