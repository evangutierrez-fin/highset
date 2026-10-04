# 0013. Methodology: adaptive spec-driven development with Spec Kit–compatible artifacts

- Status: Accepted
- Date: 2026-10-03
- Source: questionnaire Q8.1–Q8.7, Q13.10 (`docs/inputs/questionnaire-2026-10-03.md`)

## Context

Agents perform better with a clear spec (Q8.1). The owner wants gates on spec, plan and diff (Q8.2), guide-level strictness (Q8.4), and alignment with the methodology professionals use today (Q13.10).

## Decision

Built-in flows: `feature` (Spec → Plan → Implement → Verify) and `fix` (Plan → Implement → Verify), defined as TOML. Gates are human-only; strictness defaults to `guide` (skips allowed with a logged reason). Spec artifacts follow GitHub Spec Kit's layout (`specs/NNN-slug/spec.md`, `plan.md`, `tasks.md`) and HighSet reads existing Spec Kit projects. Verification combines project checks, acceptance criteria and a reviewer agent. Details in `specs/methodology.md`.

## Consequences

+ Recognizable to anyone using Spec Kit or Kiro-style workflows; agents get structure.
- Small changes would be slowed by ceremony; the `fix` flow and guide mode prevent that.

## Alternatives considered

Kanban without phases (no method, pain P4). Strict SDD always (too heavy for small fixes). TDD-only (not suitable for research/data work).
