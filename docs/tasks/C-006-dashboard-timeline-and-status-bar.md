---
id: C-006
title: "Dashboard, timeline and status bar"
lane: tui
milestone: M4
size: M
status: todo
depends_on: [C-003, A-009]
requirements: [UX-4, AGT-9, COST-1]
open_questions: []
---

# C-006 · Dashboard, timeline and status bar

**Lane:** tui · **Milestone:** M4 · **Size:** M · **Depends on:** [C-003](C-003-sidebar-task-list-kanban-and-task-detail.md), [A-009](A-009-attention-notifications-and-hook-signals.md)

## Goal

The Superset-style overview: every agent, every project, everything waiting.

## Read first

- [`specs/tui.md#2-views-q95`](../specs/tui.md#2-views-q95)
- ADR [0014](../decisions/0014-terminal-ui-ratatui-ide-layout-hybrid-keymap.md)
- PRD requirements: UX-4, AGT-9, COST-1

## Scope

- Dashboard of sessions across all projects, attention queue with jump-to, tasks by status, costs (A-010 data when available).
- Timeline view with filters; status bar counters.

## Acceptance criteria

- [ ] Live updates through events (test with fake sessions).
- [ ] Enter on an attention item opens the right session or permission prompt.
- [ ] Snapshots in en and es.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
