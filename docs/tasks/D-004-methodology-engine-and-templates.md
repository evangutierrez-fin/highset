---
id: D-004
title: "Methodology engine and templates"
lane: platform
milestone: M3
size: M
status: todo
depends_on: [P1-007]
requirements: [MTH-1, MTH-4, MTH-5]
open_questions: []
---

# D-004 · Methodology engine and templates

**Lane:** platform · **Milestone:** M3 · **Size:** M · **Depends on:** [P1-007](P1-007-tasks-quick-capture-and-ui-bootstrap.md)

## Goal

Encode the method as data that HighSet can guide users through.

## Read first

- [`specs/methodology.md#1-flows`](../specs/methodology.md#1-flows)
- [`specs/methodology.md#2-spec-artifacts-spec-kitcompatible`](../specs/methodology.md#2-spec-artifacts-spec-kitcompatible)
- [`specs/methodology.md#6-templates-mth-5`](../specs/methodology.md#6-templates-mth-5)
- ADR [0013](../decisions/0013-methodology-adaptive-spec-driven-development.md)
- ADR [0018](../decisions/0018-follow-industry-standards-before-inventing.md)
- PRD requirements: MTH-1, MTH-4, MTH-5

## Scope

- Flow TOML loader and validation (move the built-ins from core here), override order.
- Templates: spec, plan, tasks (Spec Kit–compatible; verify the current Spec Kit templates first), ADR (MADR), PR, retro; `minijinja` with strict undefined variables.
- Spec Kit detection (`.specify/` → constitution as an always-on source); `highset task next`.

## Acceptance criteria

- [ ] Flow validation tests: unknown status, unreachable phase, missing gate.
- [ ] Templates render with all variables; an undefined variable is an error.
- [ ] A fixture with `.specify/` makes the constitution an always-on instruction source.
- [ ] `task next` gives a suggestion for every status in both flows (snapshot).
- [ ] Verified facts records the Spec Kit and MADR versions used.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
