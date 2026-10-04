---
id: P0-005
title: "Errors, logging and i18n scaffold"
lane: infra
milestone: M1
size: M
status: todo
depends_on: [P0-001]
requirements: [UX-9, NFR-I18N-1]
open_questions: []
---

# P0-005 · Errors, logging and i18n scaffold

**Lane:** infra · **Milestone:** M1 · **Size:** M · **Depends on:** [P0-001](P0-001-bootstrap-the-cargo-workspace.md)

## Goal

Shared conventions for errors, logs and translated text before features start producing them.

## Read first

- [`specs/tui.md#6-i18n-ux-9`](../specs/tui.md#6-i18n-ux-9)
- [`specs/security.md#5-network-and-privacy`](../specs/security.md#5-network-and-privacy)
- [`ARCHITECTURE.md#13-i18n`](../ARCHITECTURE.md#13-i18n)
- ADR [0014](../decisions/0014-terminal-ui-ratatui-ide-layout-hybrid-keymap.md)
- PRD requirements: UX-9, NFR-I18N-1

## Scope

- `tracing` setup per role (daemon, tui, cli, mcp): file logs in `state/logs/<role>.log`, daily rotation, 14-day retention, level from `HIGHSET_LOG`; the TUI never logs to stdout.
- Panic hook that logs the panic with a backtrace and exposes a hook point for terminal restore (used by C-001).
- Error conventions: one `thiserror` enum per crate, plus a `UserFacing` trait that yields a Fluent key and args for hints.
- `highset-i18n`: Fluent bundles embedded at compile time for `en` and `es`; a lookup helper with args; language resolution (`ui.language` auto/en/es → `LC_ALL`/`LANG` → en).
- `xtask check-i18n`: every key exists in both locales and no key is unused; wired into CI.
- Translate the CLI's top-level `--help` texts.

## Acceptance criteria

- [ ] `HIGHSET_LOG=debug highset --version` writes a log file in the state dir and prints nothing extra.
- [ ] `LANG=es_MX.UTF-8 highset --help` shows Spanish descriptions; `LANG=en_US.UTF-8` shows English; an unknown locale falls back to English.
- [ ] Removing a key from `es` makes `xtask check-i18n` fail.
- [ ] A forced panic in a test binary is logged with a backtrace.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
