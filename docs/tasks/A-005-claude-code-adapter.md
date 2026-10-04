---
id: A-005
title: "Claude Code adapter"
lane: agents
milestone: M2
size: L
status: todo
depends_on: [A-003, A-004, A-009]
requirements: [AGT-1, AGT-2]
open_questions: []
---

# A-005 · Claude Code adapter

**Lane:** agents · **Milestone:** M2 · **Size:** L · **Depends on:** [A-003](A-003-pty-runner-and-terminal-streaming.md), [A-004](A-004-acp-client.md), [A-009](A-009-attention-notifications-and-hook-signals.md)

## Goal

First-class support for the owner's main agent.

## Read first

- [`specs/agents.md#5-per-agent-adapter-notes-hypotheses-to-verify`](../specs/agents.md#5-per-agent-adapter-notes-hypotheses-to-verify)
- [`specs/agents.md#6-transcript-harvesters`](../specs/agents.md#6-transcript-harvesters)
- ADR [0005](../decisions/0005-agent-integration-acp-first-embedded-pty.md)
- PRD requirements: AGT-1, AGT-2

## Scope

- Spike first (≤ 2 h): verify the binary, the ACP adapter package and how to launch it, headless flags, config files and precedence, hooks usable for attention, log locations. Record everything in agents.md Verified facts.
- Detection with version; modes ACP (if the adapter is installed) → PTY → headless.
- Per-session launch injection (flags/env) for instructions, MCP config and settings, without modifying the user's global Claude Code config.
- Headless stream-json parser → `SessionEvent`s.
- Transcript harvester for usage and messages.
- Attention bridge: Claude Code hooks call `highset hook signal` (the settings emission itself is B-007).

## Acceptance criteria

- [ ] The Claude Code Verified facts section is filled with date, version and sources.
- [ ] Parser and harvester tests pass on committed, redacted fixtures.
- [ ] Manual test (documented): a real Claude Code session in ACP mode and in PTY mode on a task, visible through `session log`.
- [ ] If Claude Code or its ACP adapter is missing, `agent list` shows an install hint and the modes fall back correctly.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
