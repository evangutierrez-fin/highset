---
id: P0-003
title: "Open-source repository scaffolding"
lane: infra
milestone: M1
size: S
status: todo
depends_on: [P0-001]
requirements: [NFR-OSS-1]
open_questions: []
---

# P0-003 · Open-source repository scaffolding

**Lane:** infra · **Milestone:** M1 · **Size:** S · **Depends on:** [P0-001](P0-001-bootstrap-the-cargo-workspace.md)

## Goal

Make the repository ready to be public, except for the license decision.

## Read first

- [`PRD.md#82-other`](../PRD.md#82-other)
- ADR [0023](../decisions/0023-license.md)
- PRD requirements: NFR-OSS-1

## Scope

- Keep README.md, add a Development section; add `README.es.md` (Spanish).
- `CONTRIBUTING.md`: how work is organized (docs/tasks, ADRs, Definition of Done), commands, commit conventions.
- `CODE_OF_CONDUCT.md` (Contributor Covenant 2.1) and `SECURITY.md` (private reporting through GitHub Security Advisories).
- `.github/ISSUE_TEMPLATE/{bug,feature}.yml` and `.github/pull_request_template.md` with the DoD checklist from AGENTS.md.
- `CHANGELOG.md` in Keep a Changelog format with an `Unreleased` section.
- `LICENSE` (MIT, ADR 0023) already exists at the root: keep it, and reference it from README and the Cargo metadata.

## Acceptance criteria

- [ ] All files exist and render correctly as Markdown.
- [ ] The PR template contains the full Definition of Done from AGENTS.md.
- [ ] All relative links in README files resolve (checked by a script or logged manually).
- [ ] README's License section states MIT and links `LICENSE`.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
