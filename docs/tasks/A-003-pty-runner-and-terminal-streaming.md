---
id: A-003
title: "PTY runner and terminal streaming"
lane: agents
milestone: M2
size: L
status: todo
depends_on: [A-002]
requirements: [AGT-5, UX-5, PERF-2]
open_questions: []
---

# A-003 · PTY runner and terminal streaming

**Lane:** agents · **Milestone:** M2 · **Size:** L · **Depends on:** [A-002](A-002-agent-adapter-framework-and-session-supervisor.md)

## Goal

Run any CLI agent inside an embedded terminal that clients can attach to and leave at will.

## Read first

- [`specs/agents.md#32-pty-fallback-for-every-agent`](../specs/agents.md#32-pty-fallback-for-every-agent)
- [`specs/daemon-protocol.md#5-session-output-streaming`](../specs/daemon-protocol.md#5-session-output-streaming)
- ADR [0005](../decisions/0005-agent-integration-acp-first-embedded-pty.md)
- PRD requirements: AGT-5, UX-5, PERF-2

## Scope

- `portable-pty` spawn; per-session ring buffer; `vt100` parser for snapshots.
- `session.attach` (snapshot + sequence-numbered `session.output`), `detach`, `input`, `resize`; output coalesced to ≤ 60 notifications/s per client; `session.resync` on lag.
- Idle heuristic emitting `InputRequested` (agents §3.2).

## Acceptance criteria

- [ ] Fake agent `--pty`: attach, detach and reattach reproduce the screen (snapshot test).
- [ ] Input round trip and resize (the child sees the new size) are tested.
- [ ] 10 MB of output does not grow daemon RSS beyond the buffer + 10 MB (measured).
- [ ] Idle sessions cost ~0 CPU (no busy loops; reviewer check + measurement).
- [ ] `InputRequested` fires once per idle period for the fake prompt.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
