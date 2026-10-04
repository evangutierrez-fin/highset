---
id: C-004
title: "Live session view"
lane: tui
milestone: M2
size: L
status: todo
depends_on: [C-001, A-003, A-004]
requirements: [AGT-5, AGT-6, UX-5, PERF-2]
open_questions: []
---

# C-004 · Live session view

**Lane:** tui · **Milestone:** M2 · **Size:** L · **Depends on:** [C-001](C-001-tui-shell-state-architecture-and-layout.md), [A-003](A-003-pty-runner-and-terminal-streaming.md), [A-004](A-004-acp-client.md)

## Goal

Watch and steer agents live without leaving HighSet.

## Read first

- [`specs/tui.md#4-live-session-pane`](../specs/tui.md#4-live-session-pane)
- ADR [0005](../decisions/0005-agent-integration-acp-first-embedded-pty.md)
- ADR [0014](../decisions/0014-terminal-ui-ratatui-ide-layout-hybrid-keymap.md)
- PRD requirements: AGT-5, AGT-6, UX-5, PERF-2

## Scope

- Session list; start-agent dialog (agent, profile, mode, prompt) through `session.start` (switches to `task.start` when D-005 lands).
- PTY pane with a vt100 terminal widget, escape chord, attach/detach, resync.
- ACP structured view with permission prompt keys; quick session switching.

## Acceptance criteria

- [ ] With the fake agent, typing and resizing work in the PTY pane, and the escape chord returns focus.
- [ ] Approving or denying an ACP permission from the TUI resolves the request.
- [ ] With four sessions flooding output, keypress → frame stays ≤ 16 ms p95 (bench).
- [ ] Snapshots of ACP view states: message, tool call, permission prompt.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
