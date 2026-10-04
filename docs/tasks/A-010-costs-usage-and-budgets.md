---
id: A-010
title: "Costs, usage and budgets"
lane: agents
milestone: M4
size: M
status: todo
depends_on: [A-008]
requirements: [COST-1, COST-2, COST-3, COST-4]
open_questions: []
---

# A-010 · Costs, usage and budgets

**Lane:** agents · **Milestone:** M4 · **Size:** M · **Depends on:** [A-008](A-008-session-recording.md)

## Goal

Know what every session, task and project costs, and stop surprises.

## Read first

- [`specs/costs.md`](../specs/costs.md)
- PRD requirements: COST-1, COST-2, COST-3, COST-4

## Scope

- Usage ingestion and deduplication from adapters, harvesters and LLM tasks.
- `pricing.toml` defaults (refresh Anthropic prices from the official pricing page; record `last_verified`).
- Billing modes (api/subscription); aggregates by session/task/project/day/agent/model.
- Budgets with 80%/100% alerts and optional pause; `--override-budget` with reason.
- CLI `highset cost`, `highset budget set`; today's cost in `ui.bootstrap`.

## Acceptance criteria

- [ ] Aggregation tests with fixtures for every grouping.
- [ ] Each threshold alert fires once per period; `pause` cancels a running fake session and refuses new ones without an override.
- [ ] Subscription mode never shows currency (CLI output tests).
- [ ] An unknown model counts tokens, reports cost as unknown and shows a hint.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
