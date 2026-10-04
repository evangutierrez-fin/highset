---
id: D-011
title: "Backups and doctor"
lane: platform
milestone: M4
size: S
status: todo
depends_on: [P1-005]
requirements: [DATA-5]
open_questions: []
---

# D-011 · Backups and doctor

**Lane:** platform · **Milestone:** M4 · **Size:** S · **Depends on:** [P1-005](P1-005-daemon-skeleton-and-client-autostart.md)

## Goal

Recover from mistakes and diagnose problems without help.

## Read first

- [`specs/storage.md#5-backups-d-011`](../specs/storage.md#5-backups-d-011)
- ADR [0007](../decisions/0007-plain-files-are-the-source-of-truth-sqlite-and.md)
- PRD requirements: DATA-5

## Scope

- Daily snapshot with retention; `highset backup now|list|restore`.
- `highset doctor`: git version, agents and ACP adapters, gh, keychain, socket, disk space, config validity, MCP server commands on PATH, startup-time sanity.

## Acceptance criteria

- [ ] Backup → restore round trip (dry run, and a real restore into a temp dir).
- [ ] Retention policy unit tests.
- [ ] Every failing doctor check has a fix hint (en/es); `--json` supported.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
