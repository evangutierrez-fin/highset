# 0015. Search: tantivy full-text and nucleo fuzzy; no embeddings in v0.1

- Status: Accepted
- Date: 2026-10-03
- Source: questionnaire Q4.7, Q5.8 (`docs/inputs/questionnaire-2026-10-03.md`)

## Context

The owner wants fuzzy, full-text and metadata filters (Q4.7) and no semantic search in v1 (Q5.8).

## Decision

tantivy index in the state dir, updated incrementally from events, with metadata filters. nucleo for fuzzy matching in the palette and mention picker. Embeddings are out of scope until measurements show a need. Details in `specs/search.md`.

## Consequences

+ Fast, local, no model dependency.
- Queries must use words that appear in the documents; acceptable for v0.1.

## Alternatives considered

Embeddings with local models (heavier, adds compute; contradicts Q13.9 for v0.1). SQLite FTS5 only (weaker ranking and filters; kept as a fallback).
