---
id: P1-004
title: "Protocol crate v1"
lane: core
milestone: M1
size: M
status: todo
depends_on: [P1-001]
requirements: [INT-2, UX-10]
open_questions: []
---

# P1-004 · Protocol crate v1

**Lane:** core · **Milestone:** M1 · **Size:** M · **Depends on:** [P1-001](P1-001-domain-model-and-contextbundle-ir.md)

## Goal

A typed, versioned JSON-RPC contract shared by every client and the daemon.

## Read first

- [`specs/daemon-protocol.md`](../specs/daemon-protocol.md)
- ADR [0004](../decisions/0004-local-daemon-with-thin-clients-over-a-unix.md)
- PRD requirements: INT-2, UX-10

## Scope

- JSON-RPC 2.0 types; newline-delimited framing with an 8 MiB frame limit.
- `initialize` handshake: protocol version, client kind, optional session token.
- Typed request/response structs for the M1 methods in daemon-protocol §4 (daemon, events, ui.bootstrap, config, workspace, project, task list/get/create/update/move/remove/capture). Later lanes add their methods additively.
- Error code table (daemon-protocol §3) with `hint_key` in `error.data`.
- Async client: connect, multiplex requests by id, notification stream, reconnect helper.

## Acceptance criteria

- [ ] Serde round-trip test for every method type and event.
- [ ] Loopback test (in-memory duplex) between the client and a mock router.
- [ ] A protocol version mismatch yields a clear error suggesting `highset daemon restart`.
- [ ] A test or xtask check verifies that daemon-protocol.md lists every implemented method.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
