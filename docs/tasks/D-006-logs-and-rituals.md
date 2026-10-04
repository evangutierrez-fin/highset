---
id: D-006
title: "Logs and rituals"
lane: platform
milestone: M4
size: M
status: todo
depends_on: [D-004, A-008]
requirements: [MTH-6, MTH-7]
open_questions: []
---

# D-006 · Logs and rituals

**Lane:** platform · **Milestone:** M4 · **Size:** M · **Depends on:** [D-004](D-004-methodology-engine-and-templates.md), [A-008](A-008-session-recording.md)

## Goal

Nothing is forgotten: decisions, changes and daily progress are written down automatically.

## Read first

- [`specs/methodology.md#7-logs-mth-7`](../specs/methodology.md#7-logs-mth-7)
- [`specs/methodology.md#8-rituals-mth-6`](../specs/methodology.md#8-rituals-mth-6)
- ADR [0013](../decisions/0013-methodology-adaptive-spec-driven-development.md)
- ADR [0018](../decisions/0018-follow-industry-standards-before-inventing.md)
- PRD requirements: MTH-6, MTH-7

## Scope

- ADR creation (`highset adr new`, from approved proposals), changelog entries, journal writer.
- `highset standup` and `highset review weekly`; a data API for the TUI's first-open-of-the-day standup.

## Acceptance criteria

- [ ] Golden standup and weekly review generated from a fixture event log.
- [ ] ADR numbering stays correct under concurrent creation.
- [ ] Changelog entries are grouped per Keep a Changelog sections.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
