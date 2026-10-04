---
id: A-008
title: "Session recording"
lane: agents
milestone: M2
size: M
status: todo
depends_on: [A-002, D-001]
requirements: [AGT-8, DATA-3]
open_questions: []
---

# A-008 · Session recording

**Lane:** agents · **Milestone:** M2 · **Size:** M · **Depends on:** [A-002](A-002-agent-adapter-framework-and-session-supervisor.md), [D-001](D-001-secrets-and-redaction.md)

## Goal

Every session leaves a complete, redacted, replayable record.

## Read first

- [`specs/storage.md#24-session-directory-contentsessionsproject-idsession-id`](../specs/storage.md#24-session-directory-contentsessionsproject-idsession-id)
- [`specs/security.md#3-redaction-data-3`](../specs/security.md#3-redaction-data-3)
- ADR [0007](../decisions/0007-plain-files-are-the-source-of-truth-sqlite-and.md)
- ADR [0016](../decisions/0016-secrets-in-the-os-keychain-redact-before.md)
- PRD requirements: AGT-8, DATA-3

## Scope

- `transcript.jsonl` writer (every event except raw output; ANSI-stripped PTY text in ≤ 8 KiB chunks); optional raw PTY `zstd` file.
- `diff.patch` at session end (A-001), `usage.jsonl`, final `session.toml` update.
- Redaction (D-001) on every write; buffered writes that never block event delivery.
- Hooks for later consumers: summary placeholder (D-003), search indexing events (D-007).

## Acceptance criteria

- [ ] The session directory layout matches storage.md §2.4 (test).
- [ ] Replaying `transcript.jsonl` reproduces the event sequence (test).
- [ ] A canary secret printed by the fake agent never appears in any session file (end-to-end test).
- [ ] A flood scenario shows writes do not delay event delivery (bench).
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
