# 0008. Storage locations: content vs. state, global vs. project vs. local

- Status: Accepted
- Date: 2026-10-03
- Source: questionnaire Q5.3, Q10.4 (`docs/inputs/questionnaire-2026-10-03.md`)

## Context

Knowledge and configuration exist at personal and project levels (Q5.3); some project knowledge must stay private (Q5.3 note); users may want to sync their personal data (Q10.4 note).

## Decision

`HIGHSET_HOME` (default `~/.highset`) contains `content/` (plain files, relocatable, sync-safe), `state/` (caches, socket, logs; never synced), `worktrees/` and `backups/`. Each project has a committed `.highset/` and a gitignored `.highset/local/` for private items and personal overrides. Layout in `ARCHITECTURE.md` §4.

## Consequences

+ Clear rules for what can be synced or committed.
+ Private-per-project knowledge without leaking into the repo.
- Two places to look for project data (`.highset/` and `content/sessions/`); `context explain` and the TUI hide this complexity.

## Alternatives considered

XDG-only layout (less discoverable on macOS; can be added later via env vars). Everything inside the repo (transcripts would bloat and leak).
