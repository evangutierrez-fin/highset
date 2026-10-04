---
id: P1-002
title: "Content store: Markdown and TOML files"
lane: core
milestone: M1
size: M
status: todo
depends_on: [P1-001]
requirements: [ORG-3, CTX-1, NFR-REL-1]
open_questions: []
---

# P1-002 · Content store: Markdown and TOML files

**Lane:** core · **Milestone:** M1 · **Size:** M · **Depends on:** [P1-001](P1-001-domain-model-and-contextbundle-ir.md)

## Goal

Read and write every content file format safely, preserving whatever the user wrote.

## Read first

- [`specs/storage.md#1-principles`](../specs/storage.md#1-principles)
- [`specs/storage.md#2-file-formats`](../specs/storage.md#2-file-formats)
- ADR [0007](../decisions/0007-plain-files-are-the-source-of-truth-sqlite-and.md)
- ADR [0011](../decisions/0011-file-formats-toml-for-config-markdown-yaml.md)
- PRD requirements: ORG-3, CTX-1, NFR-REL-1

## Scope

- Parsers and writers for config/workspace/project TOML, task files, knowledge items, packs, profiles (`*.agent.md`), prompts, proposals, `session.toml`, and JSONL appenders (transcript, usage).
- YAML frontmatter parse/serialize that preserves unknown keys and key order; the body is preserved byte for byte.
- `toml_edit` for TOML files, so comments and formatting survive.
- Atomic write helper: temp file, fsync, rename, dir fsync.
- Task number allocation by scanning (max + 1); slug generation (ASCII-folded, ≤ 40 chars).
- Global inbox items stored as task files in `content/tasks/I-0001-<slug>.md` (document this in storage.md §2.5).
- A `ContentStore` API used only by the daemon (single writer).

## Acceptance criteria

- [ ] Golden fixtures for each format in `crates/highset-store/tests/fixtures/`, matching the examples in storage.md §2.
- [ ] Reading and rewriting an unchanged file is byte-identical; changing one field changes only that field's lines.
- [ ] Unknown frontmatter keys and TOML comments survive edits.
- [ ] Invalid frontmatter returns a typed error with path and line.
- [ ] A simulated crash between temp write and rename (test hook) never leaves a truncated target file.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
