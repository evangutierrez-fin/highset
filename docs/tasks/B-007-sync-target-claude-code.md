---
id: B-007
title: "Sync target: Claude Code"
lane: context
milestone: M3
size: M
status: todo
depends_on: [B-006]
requirements: [CTX-7, HRN-2, HRN-5]
open_questions: []
---

# B-007 · Sync target: Claude Code

**Lane:** context · **Milestone:** M3 · **Size:** M · **Depends on:** [B-006](B-006-context-compiler.md)

## Goal

Claude Code receives HighSet's instructions, MCP servers, permissions, skills and hooks automatically.

## Read first

- [`specs/context-compiler.md#3-sync-targets-ctx-7-hrn-2`](../specs/context-compiler.md#3-sync-targets-ctx-7-hrn-2)
- [`specs/harness.md#5-compilation-per-agent`](../specs/harness.md#5-compilation-per-agent)
- ADR [0009](../decisions/0009-highset-is-the-source-of-truth-for-harness-and.md)
- PRD requirements: CTX-7, HRN-2, HRN-5

## Scope

- Spike: verify the current formats of CLAUDE.md imports, `.claude/settings.json` (permissions, hooks), `.claude/agents`, `.claude/skills`, `.claude/commands`, `.mcp.json`.
- Emit them per context-compiler §3 with managed blocks and the ownership manifest; launch-mode artifacts for restricted content.
- Permission compilation to Claude Code syntax; attention-bridge hooks.
- `highset sync [--dry-run]`.

## Acceptance criteria

- [ ] Dry-run shows a unified diff; applying and re-running produces no changes (idempotent).
- [ ] User content outside the managed block and user-owned settings keys stays untouched (fixture test).
- [ ] No secret values appear in any written file (scan test with canary secrets).
- [ ] Permission compilation unit tests; lossy mappings produce warnings.
- [ ] Manual: a real Claude Code session picks up instructions, MCP server and permissions (documented).
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
