---
id: A-006
title: "Codex CLI adapter"
lane: agents
milestone: M3
size: L
status: todo
depends_on: [A-003, A-004, A-009]
requirements: [AGT-1, AGT-2, HRN-2]
open_questions: []
---

# A-006 · Codex CLI adapter

**Lane:** agents · **Milestone:** M3 · **Size:** L · **Depends on:** [A-003](A-003-pty-runner-and-terminal-streaming.md), [A-004](A-004-acp-client.md), [A-009](A-009-attention-notifications-and-hook-signals.md)

## Goal

Support Codex CLI without ever breaking the user's own Codex setup.

## Read first

- [`specs/agents.md#5-per-agent-adapter-notes-hypotheses-to-verify`](../specs/agents.md#5-per-agent-adapter-notes-hypotheses-to-verify)
- ADR [0005](../decisions/0005-agent-integration-acp-first-embedded-pty.md)
- ADR [0009](../decisions/0009-highset-is-the-source-of-truth-for-harness-and.md)
- PRD requirements: AGT-1, AGT-2, HRN-2

## Scope

- Spike: verify binary, ACP adapter (`codex-acp` or successor), headless mode (`codex exec --json` or successor), config file format and location, `notify` hook, session log location.
- Decide the config injection strategy (per-session `CODEX_HOME` overlay that preserves auth vs. project-level config) and record it in a new ADR.
- Modes ACP → PTY → headless; headless parser; rollout harvester; attention bridge via `notify`.

## Acceptance criteria

- [ ] The ADR for the injection strategy exists and is Accepted.
- [ ] The user's existing Codex config and auth are untouched (test with a temp home containing fixture config and auth files).
- [ ] Parser and harvester tests pass on redacted fixtures.
- [ ] Manual test with a real Codex session documented; Verified facts filled.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
