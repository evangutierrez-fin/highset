---
id: P1-003
title: "SQLite cache, migrations and file watcher"
lane: core
milestone: M1
size: M
status: todo
depends_on: [P1-002]
requirements: [DATA-1, PERF-9]
open_questions: []
---

# P1-003 · SQLite cache, migrations and file watcher

**Lane:** core · **Milestone:** M1 · **Size:** M · **Depends on:** [P1-002](P1-002-content-store-markdown-and-toml-files.md)

## Goal

Fast queries over everything, always rebuildable from files, with live updates when files change outside HighSet.

## Read first

- [`specs/storage.md#3-sqlite-cache-statehighsetdb`](../specs/storage.md#3-sqlite-cache-statehighsetdb)
- [`specs/storage.md#4-file-watcher`](../specs/storage.md#4-file-watcher)
- ADR [0007](../decisions/0007-plain-files-are-the-source-of-truth-sqlite-and.md)
- PRD requirements: DATA-1, PERF-9

## Scope

- Schema from storage.md §3 (finalize it and document the final DDL there); embedded migrations; WAL mode.
- Store actor: a dedicated thread owning the connection; async API through channels; prepared-statement cache.
- Full reindex from content (`highset reindex`, local until P1-005).
- Incremental indexer and `notify` watcher (200 ms debounce) over the content dir and registered projects; suppress events caused by the daemon's own writes.
- Publishes `Event`s through a callback interface (wired to the bus in P1-005).

## Acceptance criteria

- [ ] Deleting `highset.db` and reindexing yields the same query results as the incrementally maintained cache (equivalence test on a generated dataset).
- [ ] An external edit of a task file is reflected in the cache within 1 s (integration test).
- [ ] Reindexing 10k tasks + 10k knowledge items takes ≤ 10 s in release mode (measured and logged).
- [ ] An invalid external edit emits a validation error event and keeps the previous cached row.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
