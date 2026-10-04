---
id: C-001
title: "TUI shell, state architecture and layout"
lane: tui
milestone: M2
size: L
status: todo
depends_on: [P1-005, P0-005]
requirements: [UX-1, UX-7, UX-8, PERF-1, PERF-2]
open_questions: []
---

# C-001 · TUI shell, state architecture and layout

**Lane:** tui · **Milestone:** M2 · **Size:** L · **Depends on:** [P1-005](P1-005-daemon-skeleton-and-client-autostart.md), [P0-005](P0-005-errors-logging-and-i18n-scaffold.md)

## Goal

A fast, stable foundation for every view.

## Read first

- [`specs/tui.md#1-layout-ide-style-q91`](../specs/tui.md#1-layout-ide-style-q91)
- [`specs/tui.md#5-architecture-and-rendering-perf-12`](../specs/tui.md#5-architecture-and-rendering-perf-12)
- ADR [0014](../decisions/0014-terminal-ui-ratatui-ide-layout-hybrid-keymap.md)
- ADR [0017](../decisions/0017-performance-budgets-are-release-blocking.md)
- PRD requirements: UX-1, UX-7, UX-8, PERF-1, PERF-2

## Scope

- Reducer/effects architecture, `select!` event loop with render-on-dirty, daemon client, `ui.bootstrap` first frame.
- Layout regions with collapse rules, focus model, ANSI theme roles, optional mouse.
- Panic restore, `$EDITOR` suspend/resume; `highset` and `highset tui` entry points.

## Acceptance criteria

- [ ] First frame ≤ 150 ms with the daemon running on the 10k dataset (measured).
- [ ] Zero renders during 5 s of idle (test hook counter).
- [ ] Snapshot tests at 80×24, 120×40 and 200×60, in en and es.
- [ ] The manual terminal checklist (tui §7) is documented.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
