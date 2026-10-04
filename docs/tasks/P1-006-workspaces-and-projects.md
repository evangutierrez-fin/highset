---
id: P1-006
title: "Workspaces and projects"
lane: core
milestone: M1
size: M
status: todo
depends_on: [P1-005]
requirements: [ORG-1, ORG-2]
open_questions: []
---

# P1-006 · Workspaces and projects

**Lane:** core · **Milestone:** M1 · **Size:** M · **Depends on:** [P1-005](P1-005-daemon-skeleton-and-client-autostart.md)

## Goal

Register where work happens.

## Read first

- [`specs/cli.md`](../specs/cli.md)
- [`specs/storage.md#21-configtoml-global-workspacetoml-projecttoml`](../specs/storage.md#21-configtoml-global-workspacetoml-projecttoml)
- ADR [0008](../decisions/0008-storage-locations-content-vs-state-global-vs.md)
- PRD requirements: ORG-1, ORG-2

## Scope

- Services and CLI: `highset ws list|add|rename|rm`, `highset project list|add [path] [--ws] [--name]|show|link|rm`.
- `project add` creates the `.highset/` skeleton (project.toml, context/, knowledge/, packs/, tasks/, local/, plus a `.gitignore` that ignores `local/`), detects the default branch, and detects verify commands for Cargo, npm/pnpm/yarn, pytest/uv and go (best effort, editable).
- Project discovery from the current directory for all commands.
- A default workspace `personal` created on first run.

## Acceptance criteria

- [ ] `project add` on a fresh repo creates exactly the documented skeleton (snapshot of the tree).
- [ ] Re-adding is idempotent; removing a project never deletes repository files.
- [ ] Verify-command detection is unit-tested per ecosystem with fixture repos.
- [ ] All output is localized (en/es) and available with `--json`.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
