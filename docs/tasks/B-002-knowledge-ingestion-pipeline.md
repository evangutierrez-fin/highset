---
id: B-002
title: "Knowledge ingestion pipeline"
lane: context
milestone: M3
size: M
status: todo
depends_on: [B-001]
requirements: [CTX-6]
open_questions: []
---

# B-002 · Knowledge ingestion pipeline

**Lane:** context · **Milestone:** M3 · **Size:** M · **Depends on:** [B-001](B-001-knowledge-base-and-convention-files.md)

## Goal

Turn files, web pages, PDFs, images and data into useful knowledge items.

## Read first

- [`specs/knowledge.md#5-ingestion-ctx-6`](../specs/knowledge.md#5-ingestion-ctx-6)
- ADR [0007](../decisions/0007-plain-files-are-the-source-of-truth-sqlite-and.md)
- PRD requirements: CTX-6

## Scope

- Ingestion per knowledge §5: Markdown/text, code, URL → Markdown, PDF → text, image, CSV/JSON with schema preview.
- Evaluate and choose crates for readability extraction, HTML → Markdown and PDF text; record the choice in Verified facts.
- Dedupe by content hash; 20 MiB limit without `--force`; network only for URL ingest.

## Acceptance criteria

- [ ] Fixtures (HTML article, PDF, PNG, CSV, JSON, code file) produce the expected items (golden tests; URLs served by a local test server).
- [ ] Re-adding identical content updates the existing item instead of duplicating it.
- [ ] Files over 20 MiB are rejected without `--force`, with a hint.
- [ ] No network call happens except for URL ingest (test HTTP client fails on unexpected requests).
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
