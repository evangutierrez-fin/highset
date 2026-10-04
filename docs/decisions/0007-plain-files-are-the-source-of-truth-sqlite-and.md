# 0007. Plain files are the source of truth; SQLite and tantivy are caches

- Status: Accepted
- Date: 2026-10-03
- Source: questionnaire Q5.2, Q10.1, Q10.4 (`docs/inputs/questionnaire-2026-10-03.md`)

## Context

The owner chose Markdown with frontmatter plus a SQLite index (Q5.2) and wants data to be portable and syncable (Q10.4 note). Agents read files natively.

## Decision

All durable data lives in Markdown, TOML or JSONL files. SQLite (`state/highset.db`) and tantivy (`state/search/`) are rebuildable caches; `highset reindex` rebuilds them from scratch and a test asserts equivalence. Only the daemon writes; writes are atomic and round-trip safe.

## Consequences

+ Human-readable, git-friendly, editable in any editor; no lock-in.
+ Sync-friendly (ADR-0022).
- Need a file watcher and careful round-trip writing (toml_edit, frontmatter preservation).

## Alternatives considered

SQLite as source of truth (opaque, unsafe to sync). Files only without index (slow search and queries).
