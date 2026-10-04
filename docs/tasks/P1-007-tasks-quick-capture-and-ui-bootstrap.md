---
id: P1-007
title: "Tasks, quick capture and UI bootstrap"
lane: core
milestone: M1
size: M
status: todo
depends_on: [P1-006]
requirements: [ORG-3, ORG-4, ORG-5, ORG-6, UX-10, PERF-3]
open_questions: []
---

# P1-007 · Tasks, quick capture and UI bootstrap

**Lane:** core · **Milestone:** M1 · **Size:** M · **Depends on:** [P1-006](P1-006-workspaces-and-projects.md)

## Goal

The first useful workflow: capture ideas, create and move tasks, all from the CLI.

## Read first

- [`specs/core-domain.md#2-entities`](../specs/core-domain.md#2-entities)
- [`specs/core-domain.md#3-status-machine-and-flows`](../specs/core-domain.md#3-status-machine-and-flows)
- [`specs/cli.md`](../specs/cli.md)
- [`specs/storage.md#22-task-file-highsettaskst-0042-add-loginmd`](../specs/storage.md#22-task-file-highsettaskst-0042-add-loginmd)
- ADR [0013](../decisions/0013-methodology-adaptive-spec-driven-development.md)
- PRD requirements: ORG-3, ORG-4, ORG-5, ORG-6, UX-10, PERF-3

## Scope

- Services and CLI: `task add|list|show|edit|move|link|move-to`, `highset add "<text>"` (project Inbox, or the global inbox outside a project).
- Status moves validated by the flow (gates enforced as data; gate UI comes later); `--skip-reason` in guide mode, recorded in History.
- Dependencies with cycle detection; labels, priority, epic, links.
- `task edit` opens `$EDITOR`, validates on save, and reopens with the error at the top if invalid.
- `ui.bootstrap` returning workspaces, projects and task summaries (session/attention/cost fields stay empty until their lanes fill them).

## Acceptance criteria

- [ ] Every command supports `--json`; the JSON shapes are documented in cli.md.
- [ ] `highset add "x"` p95 ≤ 60 ms with the daemon running (hyperfine output in the verification log).
- [ ] Dependency cycles are rejected and the message shows the cycle.
- [ ] Moving `inbox → done` in the feature flow is rejected with a localized explanation; `--skip-reason` works in guide mode and is recorded.
- [ ] Global inbox items can be moved into a project and receive a project task number.
- [ ] `ui.bootstrap` returns in ≤ 40 ms on the 10k-task dataset.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
