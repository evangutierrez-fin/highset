---
id: B-009
title: "HighSet MCP server"
lane: context
milestone: M3
size: L
status: todo
depends_on: [B-006, P1-007]
requirements: [CTX-8, CTX-11, AGT-7]
open_questions: []
---

# B-009 · HighSet MCP server

**Lane:** context · **Milestone:** M3 · **Size:** L · **Depends on:** [B-006](B-006-context-compiler.md), [P1-007](P1-007-tasks-quick-capture-and-ui-bootstrap.md)

## Goal

Agents can pull context and report progress through standard MCP.

## Read first

- [`specs/mcp-server.md`](../specs/mcp-server.md)
- ADR [0010](../decisions/0010-highset-exposes-its-own-mcp-server-to-agents.md)
- PRD requirements: CTX-8, CTX-11, AGT-7

## Scope

- `highset mcp serve` built on the pinned `rmcp` crate (verify its current API first), token authentication, `mcp.*` daemon methods.
- Tools, resources and prompts from mcp-server.md; audience filtering; actor-limited task updates; rate limits; unscoped read-only mode.
- Auto-registration of the `highset` server in every bundle.

## Acceptance criteria

- [ ] An rmcp client test lists the tools with valid schemas and calls each one.
- [ ] Audience test: an item restricted to `claude-code` is never returned to a Codex session (search, get, resources).
- [ ] `highset_task_update` cannot set `done`; `highset_request_review` moves the task to review and raises attention.
- [ ] An invalid token is unauthorized; unscoped mode is read-only.
- [ ] Tool overhead ≤ 10 ms over the daemon call (bench).
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
