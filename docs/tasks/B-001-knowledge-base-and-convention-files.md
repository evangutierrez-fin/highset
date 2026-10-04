---
id: B-001
title: "Knowledge base and convention files"
lane: context
milestone: M3
size: M
status: todo
depends_on: [P1-003]
requirements: [CTX-1, CTX-2, CTX-4]
open_questions: []
---

# B-001 · Knowledge base and convention files

**Lane:** context · **Milestone:** M3 · **Size:** M · **Depends on:** [P1-003](P1-003-sqlite-cache-migrations-and-file-watcher.md)

## Goal

A place for everything agents should know, with files named for their purpose.

## Read first

- [`specs/knowledge.md#1-knowledge-items`](../specs/knowledge.md#1-knowledge-items)
- [`specs/knowledge.md#2-levels-where-an-item-lives`](../specs/knowledge.md#2-levels-where-an-item-lives)
- [`specs/knowledge.md#4-convention-files-q54-note`](../specs/knowledge.md#4-convention-files-q54-note)
- ADR [0007](../decisions/0007-plain-files-are-the-source-of-truth-sqlite-and.md)
- ADR [0020](../decisions/0020-knowledge-scoping-levels-plus-audience.md)
- PRD requirements: CTX-1, CTX-2, CTX-4

## Scope

- Knowledge item model and levels; convention file recognition (knowledge §4); shadowing.
- Daemon `services/knowledge`: `kb.list|get|add|update|remove|move_level`.
- CLI `highset kb list|add|show|edit|rm|mv` for local files (URL/PDF/etc. come in B-002) and `highset context ls`.
- `--level local` writes under `.highset/local/` (gitignored).

## Acceptance criteria

- [ ] Classification tests for every convention filename and for frontmatter `kind` overrides.
- [ ] Shadowing tests: local > project > workspace > global.
- [ ] `kb add --level local` leaves `git status` clean (integration test).
- [ ] External edits are re-indexed and emit `kb.changed`.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
