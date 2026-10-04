---
id: D-007
title: "Search"
lane: platform
milestone: M3
size: M
status: todo
depends_on: [P1-003]
requirements: [SRCH-1, SRCH-2, SRCH-3, PERF-7, PERF-8, PERF-9]
open_questions: []
---

# D-007 · Search

**Lane:** platform · **Milestone:** M3 · **Size:** M · **Depends on:** [P1-003](P1-003-sqlite-cache-migrations-and-file-watcher.md)

## Goal

Find anything in milliseconds.

## Read first

- [`specs/search.md`](../specs/search.md)
- ADR [0015](../decisions/0015-search-tantivy-full-text-and-nucleo-fuzzy-no.md)
- PRD requirements: SRCH-1, SRCH-2, SRCH-3, PERF-7, PERF-8, PERF-9

## Scope

- tantivy schema and incremental updates with batched commits; filters; ASCII folding.
- `search.query`, `search.fuzzy`, CLI `highset search`; rebuild on corruption; SQLite LIKE fallback.

## Acceptance criteria

- [ ] Rebuilding from content reproduces results (test).
- [ ] PERF-8 and PERF-9 measured; PERF-7 bench for the fuzzy helpers.
- [ ] Filter syntax tests, including Spanish accents matching unaccented queries.
- [ ] A corrupted index triggers a background rebuild while queries use the fallback.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
