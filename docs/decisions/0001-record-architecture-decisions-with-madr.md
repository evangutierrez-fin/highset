# 0001. Record architecture decisions with MADR

- Status: Accepted
- Date: 2026-10-03
- Source: questionnaire Q4.8, Q13.10 (`docs/inputs/questionnaire-2026-10-03.md`)

## Context

The project is built and maintained by AI agents, while the owner reviews behavior rather than code. Decisions must be durable, discoverable and binding, or agents will re-litigate them.

## Decision

Use Architecture Decision Records in `docs/decisions/`, in a short MADR format (template: `0000-template.md`). Number sequentially. ADRs are binding for builder agents. A change of decision is a new ADR that supersedes the old one; old ADRs are never edited except to mark them superseded.

## Consequences

+ Agents can find the reasoning behind every rule.
+ The same format is what HighSet will produce for its users (MTH-7), so the builder dogfoods it.
- Small overhead per decision.

## Alternatives considered

Free-form decision log in a single file: harder to link and supersede. No record: decisions get lost (the exact problem P6 HighSet solves).
