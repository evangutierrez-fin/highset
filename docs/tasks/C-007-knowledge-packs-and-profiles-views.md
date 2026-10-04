---
id: C-007
title: "Knowledge, packs and profiles views"
lane: tui
milestone: M3
size: M
status: todo
depends_on: [C-002, B-004, B-005]
requirements: [UX-4, CTX-1, CTX-5, HRN-1]
open_questions: []
---

# C-007 · Knowledge, packs and profiles views

**Lane:** tui · **Milestone:** M3 · **Size:** M · **Depends on:** [C-002](C-002-keymap-help-bar-and-command-palette.md), [B-004](B-004-context-packs.md), [B-005](B-005-harness-profiles-and-mcp-server-registry.md)

## Goal

Browse and edit what agents know and how they are configured.

## Read first

- [`specs/tui.md#2-views-q95`](../specs/tui.md#2-views-q95)
- ADR [0014](../decisions/0014-terminal-ui-ratatui-ide-layout-hybrid-keymap.md)
- PRD requirements: UX-4, CTX-1, CTX-5, HRN-1

## Scope

- Knowledge tree by level and kind, preview, level/audience badges and edits.
- Packs tab with enable/disable per project/task; profiles list with the resolved view and provenance; MCP registry tab; open in `$EDITOR`.

## Acceptance criteria

- [ ] Actions round-trip through the daemon.
- [ ] Launching the editor suspends and restores the TUI correctly (fake-editor test + manual check).
- [ ] Snapshots.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
