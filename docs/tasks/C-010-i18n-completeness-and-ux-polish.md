---
id: C-010
title: "i18n completeness and UX polish"
lane: tui
milestone: M4
size: S
status: todo
depends_on: [C-001, C-002, C-003, C-004, C-005, C-006, C-007, C-008, C-009]
requirements: [UX-9, NFR-I18N-1]
open_questions: []
---

# C-010 · i18n completeness and UX polish

**Lane:** tui · **Milestone:** M4 · **Size:** S · **Depends on:** [C-001](C-001-tui-shell-state-architecture-and-layout.md), [C-002](C-002-keymap-help-bar-and-command-palette.md), [C-003](C-003-sidebar-task-list-kanban-and-task-detail.md), [C-004](C-004-live-session-view.md), [C-005](C-005-diff-viewer-and-gate-actions.md), [C-006](C-006-dashboard-timeline-and-status-bar.md), [C-007](C-007-knowledge-packs-and-profiles-views.md), [C-008](C-008-inbox-and-plugin-manager-views.md), [C-009](C-009-prompt-composer-with-mentions.md)

## Goal

A coherent, fully bilingual interface with quiet, Linear-like chrome.

## Read first

- [`specs/tui.md`](../specs/tui.md)
- ADR [0014](../decisions/0014-terminal-ui-ratatui-ide-layout-hybrid-keymap.md)
- PRD requirements: UX-9, NFR-I18N-1

## Scope

- All TUI strings through Fluent; review of the Spanish texts; empty states; error toasts with fix hints; consistent key hints.

## Acceptance criteria

- [ ] `xtask check-i18n` passes; a check finds no hardcoded user-facing string literals in `highset-tui` render code (allow-list for exceptions).
- [ ] A Spanish snapshot set exists for every view.
- [ ] A checklist of empty states and error messages is in the verification log.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
