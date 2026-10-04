# 0014. Terminal UI: ratatui, IDE layout, hybrid keymap, Fluent i18n

- Status: Accepted
- Date: 2026-10-03
- Source: questionnaire Q2.5, Q2.6, Q9.1–Q9.8, Q13.8 (`docs/inputs/questionnaire-2026-10-03.md`)

## Context

The owner chose ratatui + crossterm (Q2.5), an IDE-like layout (Q9.1), Vim navigation plus single-key actions and also conventional keys (Q9.2 + note), a Ctrl+K palette (Q9.3), embedded terminals (Q9.4), terminal colors (Q9.7), optional mouse (Q9.8) and both English and Spanish (Q2.6). Linear and Superset are the references (Q13.8).

## Decision

Build the TUI on ratatui + crossterm with a reducer/effects architecture, render-on-dirty, virtualized lists, embedded PTY panes via a vt100-based widget, a remappable keymap where Vim/single-key and conventional bindings are active together, a nucleo-backed palette, and Fluent (`.ftl`) localization in en and es. Details in `specs/tui.md`.

## Consequences

+ Matches the owner's choices; testable with snapshots.
- Embedded terminals are the hardest UI part; budgeted and benchmarked.

## Alternatives considered

GUI (Tauri/egui): not a terminal app. tmux integration instead of embedded terminals: less control over attention and state.
