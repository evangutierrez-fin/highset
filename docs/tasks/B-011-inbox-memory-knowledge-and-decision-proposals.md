---
id: B-011
title: "Inbox: memory, knowledge and decision proposals"
lane: context
milestone: M4
size: M
status: todo
depends_on: [B-009]
requirements: [CTX-11, MTH-7]
open_questions: []
---

# B-011 · Inbox: memory, knowledge and decision proposals

**Lane:** context · **Milestone:** M4 · **Size:** M · **Depends on:** [B-009](B-009-highset-mcp-server.md)

## Goal

Agents' learnings become durable knowledge, but only after the owner approves them.

## Read first

- [`specs/knowledge.md#6-memory-and-proposals-ctx-11`](../specs/knowledge.md#6-memory-and-proposals-ctx-11)
- ADR [0020](../decisions/0020-knowledge-scoping-levels-plus-audience.md)
- PRD requirements: CTX-11, MTH-7

## Scope

- Proposal files, inbox service, CLI `highset inbox list|show|approve|edit|reject`.
- Approval effects: memory → `MEMORY.md` with provenance; knowledge → item; decision → ADR (D-006 writer behind an interface); doc_update → patch.
- Journal entries for every decision; integrate D-003 extraction output.

## Acceptance criteria

- [ ] Proposals are invisible to compilation and MCP until approved (test).
- [ ] Approve, edit-then-approve and reject flows are tested; provenance is preserved in MEMORY.md.
- [ ] Approving a decision proposal creates an ADR file.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
