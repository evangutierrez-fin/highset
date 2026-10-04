# 0004. Local daemon with thin clients over a Unix socket (JSON-RPC 2.0)

- Status: Accepted
- Date: 2026-10-03
- Source: questionnaire Q2.2, Q11.3 (`docs/inputs/questionnaire-2026-10-03.md`)

## Context

Agents must keep running when the interface closes (Q2.2). The CLI, the TUI, the MCP server and user scripts (Q11.3) all need the same live state.

## Decision

A per-user daemon owns all state and side effects. Clients (TUI, CLI, MCP shim, scripts) talk to it with JSON-RPC 2.0 over a Unix domain socket (`0600`), newline-delimited frames, versioned handshake. Clients auto-start the daemon. Details in `specs/daemon-protocol.md`.

## Consequences

+ Sessions survive closing the TUI; one source of truth; scriptable.
+ Single writer simplifies file safety.
- A long-running process must meet idle budgets (PERF-4/5).
- Protocol versioning is needed between client and daemon of different versions.

## Alternatives considered

All-in-TUI process (agents die with the UI). Daemon later (expensive rewrite). HTTP server on localhost (larger attack surface, port management).
