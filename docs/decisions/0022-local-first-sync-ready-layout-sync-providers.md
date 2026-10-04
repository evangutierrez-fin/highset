# 0022. Local-first, sync-ready layout; sync providers deferred

- Status: Accepted
- Date: 2026-10-03
- Source: questionnaire Q10.4 (+ note) (`docs/inputs/questionnaire-2026-10-03.md`)

## Context

The owner chose no sync in v1 but noted users must be able to use cloud or a synced global folder (Q10.4 note).

## Decision

v0.1 has no built-in sync. The content dir can be relocated to any synced folder (iCloud, Syncthing, a git repo) because it contains only plain files with machine-safe naming; the state dir (SQLite, index, socket) is never placed there and is rebuilt per machine. A `SyncProvider` interface is reserved for v0.2 plugins.

## Consequences

+ Users can sync today with tools they trust, with no corruption risk.
- No conflict resolution UI in v0.1; concurrent edits on two machines rely on the sync tool.

## Alternatives considered

Built-in cloud sync (scope, privacy, contradicts local-first). SQLite in synced folders (corruption).
