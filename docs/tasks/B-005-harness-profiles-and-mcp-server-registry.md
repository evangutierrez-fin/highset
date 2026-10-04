---
id: B-005
title: "Harness profiles and MCP server registry"
lane: context
milestone: M3
size: L
status: todo
depends_on: [B-003]
requirements: [HRN-1, HRN-3, HRN-4, HRN-8, AGT-10]
open_questions: []
---

# B-005 · Harness profiles and MCP server registry

**Lane:** context · **Milestone:** M3 · **Size:** L · **Depends on:** [B-003](B-003-levels-and-audience-resolution.md)

## Goal

Define agent harnesses once, layered from global to task.

## Read first

- [`specs/harness.md#1-profiles`](../specs/harness.md#1-profiles)
- [`specs/harness.md#2-layering-hrn-4`](../specs/harness.md#2-layering-hrn-4)
- [`specs/harness.md#7-mcp-server-registry-hrn-8`](../specs/harness.md#7-mcp-server-registry-hrn-8)
- ADR [0009](../decisions/0009-highset-is-the-source-of-truth-for-harness-and.md)
- ADR [0011](../decisions/0011-file-formats-toml-for-config-markdown-yaml.md)
- PRD requirements: HRN-1, HRN-3, HRN-4, HRN-8, AGT-10

## Scope

- Profile format (`*.agent.md`), lookup order, `extends`, task overrides, autonomy levels.
- Six built-in profiles (researcher, planner, builder, reviewer, debugger, documenter) with 150–400-word English bodies.
- MCP registry (`content/mcp/servers.toml`, project `.highset/mcp.toml`), enable/disable scopes, `secret:` reference validation (existence only).
- CLI `highset profile list|show|new|edit` and `highset mcp-server list|add|rm|enable|disable`.

## Acceptance criteria

- [ ] Resolution tests: precedence, `extends` merge, `+` list extension, task overrides; provenance for every field.
- [ ] Built-ins load, validate, and are overridable by same-name files.
- [ ] MCP registry enable/disable works at every scope; secret references are checked without reading values.
- [ ] `profile show --resolved` prints origins.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
