---
id: C-003
title: "Sidebar, task list, kanban and task detail"
lane: tui
milestone: M2
size: M
status: todo
depends_on: [C-002, P1-007]
requirements: [ORG-6, ORG-7, UX-4]
open_questions: []
---

# C-003 · Sidebar, task list, kanban and task detail

**Lane:** tui · **Milestone:** M2 · **Size:** M · **Depends on:** [C-002](C-002-keymap-help-bar-and-command-palette.md), [P1-007](P1-007-tasks-quick-capture-and-ui-bootstrap.md)

## Goal

See and move work at a glance.

## Read first

- [`specs/tui.md#2-views-q95`](../specs/tui.md#2-views-q95)
- ADR [0014](../decisions/0014-terminal-ui-ratatui-ide-layout-hybrid-keymap.md)
- PRD requirements: ORG-6, ORG-7, UX-4

## Scope

- Projects tree, task list with filters, kanban by flow statuses, moves, create, capture (`a`), task detail, badges (blocked, attention, dependencies).

## Acceptance criteria

- [ ] Create and move round-trip through the daemon; changes made from the CLI appear within 100 ms (event-driven test).
- [ ] With 1k tasks, no frame exceeds 16 ms (bench).
- [ ] Snapshots for kanban, list and detail in en and es.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
