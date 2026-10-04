---
id: C-008
title: "Inbox and plugin manager views"
lane: tui
milestone: M4
size: M
status: todo
depends_on: [C-002, B-011, D-008]
requirements: [UX-4, CTX-11, PLG-3]
open_questions: []
---

# C-008 · Inbox and plugin manager views

**Lane:** tui · **Milestone:** M4 · **Size:** M · **Depends on:** [C-002](C-002-keymap-help-bar-and-command-palette.md), [B-011](B-011-inbox-memory-knowledge-and-decision-proposals.md), [D-008](D-008-plugins-v1-declarative-packages.md)

## Goal

Approve agents' proposals and manage plugins safely.

## Read first

- [`specs/tui.md#2-views-q95`](../specs/tui.md#2-views-q95)
- [`specs/plugins.md`](../specs/plugins.md)
- ADR [0012](../decisions/0012-plugin-model-declarative-packages-now-out-of.md)
- PRD requirements: UX-4, CTX-11, PLG-3

## Scope

- Inbox: approve/edit/reject, with diffs for doc updates.
- Plugin manager: list, inspect, add with a permission review screen, update with a permission diff, remove.

## Acceptance criteria

- [ ] Approval flows work end to end.
- [ ] The permission review shows every declared permission before install; an update with changed permissions requires re-approval.
- [ ] Snapshots.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
