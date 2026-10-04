---
id: A-004
title: "ACP client"
lane: agents
milestone: M2
size: L
status: todo
depends_on: [A-002]
requirements: [AGT-2, AGT-6, HRN-5]
open_questions: []
---

# A-004 · ACP client

**Lane:** agents · **Milestone:** M2 · **Size:** L · **Depends on:** [A-002](A-002-agent-adapter-framework-and-session-supervisor.md)

## Goal

Understand what ACP-capable agents do and centralize their permission requests.

## Read first

- [`specs/agents.md#31-acp-preferred`](../specs/agents.md#31-acp-preferred)
- [`specs/harness.md#5-compilation-per-agent`](../specs/harness.md#5-compilation-per-agent)
- ADR [0005](../decisions/0005-agent-integration-acp-first-embedded-pty.md)
- PRD requirements: AGT-2, AGT-6, HRN-5

## Scope

- Integrate the pinned `agent-client-protocol` crate as the client: spawn over stdio, initialize, session/new with cwd and MCP servers from the bundle, session/prompt, map `session/update` to `SessionEvent`s.
- Client capabilities: file system read/write restricted to the session cwd; terminal capability if supported.
- `session/request_permission` → attention item (A-009 interface; stub until it lands) → reply; rule-based auto-approve/deny from the bundle's `PermissionSet` (harness §5).
- Cancel and kill; ACP version recorded in `session.toml`.

## Acceptance criteria

- [ ] Fake agent `--acp` scenarios cover streaming messages, a tool call with a diff, permission approved and denied (manual and rule-based), and cancellation.
- [ ] File writes outside the worktree are refused (test).
- [ ] Allow rules auto-approve, deny rules auto-reject, everything else waits for the owner (tests).
- [ ] The ACP protocol version is stored in `session.toml`, and agents.md Verified facts is updated.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Notes

Start by verifying the current ACP specification and crate API; record findings in specs/agents.md.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
