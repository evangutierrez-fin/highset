---
id: A-007
title: "opencode and Cursor CLI adapters"
lane: agents
milestone: M3
size: M
status: todo
depends_on: [A-003, A-004, A-009]
requirements: [AGT-1, AGT-2]
open_questions: []
---

# A-007 · opencode and Cursor CLI adapters

**Lane:** agents · **Milestone:** M3 · **Size:** M · **Depends on:** [A-003](A-003-pty-runner-and-terminal-streaming.md), [A-004](A-004-acp-client.md), [A-009](A-009-attention-notifications-and-hook-signals.md)

## Goal

Complete the owner's agent list (Q3.1).

## Read first

- [`specs/agents.md#5-per-agent-adapter-notes-hypotheses-to-verify`](../specs/agents.md#5-per-agent-adapter-notes-hypotheses-to-verify)
- ADR [0005](../decisions/0005-agent-integration-acp-first-embedded-pty.md)
- PRD requirements: AGT-1, AGT-2

## Scope

- Spike both agents: binaries, ACP support, headless modes, config/MCP/rules locations, hooks, logs.
- opencode: ACP if verified, PTY fallback, best-effort harvester.
- Cursor CLI: PTY and headless; ACP if available; best-effort harvester.
- List unsupported features explicitly in agents.md.

## Acceptance criteria

- [ ] Both agents are detected with versions.
- [ ] PTY sessions work for both (manual test documented).
- [ ] opencode ACP works if the spike verified support (manual test documented).
- [ ] Verified facts sections filled for both.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
