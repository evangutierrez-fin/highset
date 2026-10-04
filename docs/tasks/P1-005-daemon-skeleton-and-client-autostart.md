---
id: P1-005
title: "Daemon skeleton and client autostart"
lane: core
milestone: M1
size: M
status: todo
depends_on: [P1-003, P1-004]
requirements: [AGT-4, PERF-3, PERF-4, PERF-5]
open_questions: []
---

# P1-005 · Daemon skeleton and client autostart

**Lane:** core · **Milestone:** M1 · **Size:** M · **Depends on:** [P1-003](P1-003-sqlite-cache-migrations-and-file-watcher.md), [P1-004](P1-004-protocol-crate-v1.md)

## Goal

A long-running, idle-cheap daemon that every client reaches transparently.

## Read first

- [`specs/daemon-protocol.md#1-process-model`](../specs/daemon-protocol.md#1-process-model)
- [`specs/daemon-protocol.md#6-service-registry`](../specs/daemon-protocol.md#6-service-registry)
- ADR [0004](../decisions/0004-local-daemon-with-thin-clients-over-a-unix.md)
- ADR [0017](../decisions/0017-performance-budgets-are-release-blocking.md)
- PRD requirements: AGT-4, PERF-3, PERF-4, PERF-5

## Scope

- `highset daemon run|start|stop|restart|status`.
- Unix socket `state/run/highset.sock` (0600, directory 0700), single-instance lock, stale socket cleanup.
- Autostart from any client (daemon-protocol §1).
- Router, `Service` registry, event bus (`broadcast`) and `events.subscribe` with topic filters and lag handling.
- Graceful shutdown; scheduler skeleton (≤ 1 wakeup per minute).
- Move `config show` and `reindex` to go through the daemon.

## Acceptance criteria

- [ ] `highset daemon status` auto-starts the daemon and prints version, pid and uptime.
- [ ] A second `daemon run` exits with code 3 and "already running (pid N)".
- [ ] Socket permissions are 0600 (test).
- [ ] After `kill -9` of the daemon, the next client command recovers (stale socket and lock handled).
- [ ] Idle for 60 s: < 0.5% CPU and ≤ 60 MB RSS (measured and logged); no timers faster than 1 Hz (reviewer check).
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
