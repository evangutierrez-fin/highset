---
id: C-005
title: "Diff viewer and gate actions"
lane: tui
milestone: M4
size: M
status: todo
depends_on: [C-001, A-001]
requirements: [MTH-2, UX-4]
open_questions: []
---

# C-005 · Diff viewer and gate actions

**Lane:** tui · **Milestone:** M4 · **Size:** M · **Depends on:** [C-001](C-001-tui-shell-state-architecture-and-layout.md), [A-001](A-001-git-service-worktrees-branches-commits-diffs.md)

## Goal

Review what an agent changed and approve it in one place.

## Read first

- [`specs/tui.md#2-views-q95`](../specs/tui.md#2-views-q95)
- [`specs/methodology.md#3-gates-mth-2`](../specs/methodology.md#3-gates-mth-2)
- ADR [0014](../decisions/0014-terminal-ui-ratatui-ide-layout-hybrid-keymap.md)
- PRD requirements: MTH-2, UX-4

## Scope

- File tree, unified and side-by-side (≥ 160 columns) diff, per-file lazy loading, hunk navigation.
- Lightweight syntax highlighting only if it keeps the budgets (measure; otherwise skip and record why).
- Actions: approve diff, request changes (message to the session or a note), finish (D-005 when present).

## Acceptance criteria

- [ ] A 5k-line diff renders lazily with no frame over 16 ms (bench).
- [ ] Approving updates the gate through the daemon (integration test).
- [ ] Snapshots.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
