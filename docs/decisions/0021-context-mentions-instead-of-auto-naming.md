# 0021. Context mentions (@) instead of auto-naming

- Status: Accepted
- Date: 2026-10-03
- Source: questionnaire Q2.4 note (`docs/inputs/questionnaire-2026-10-03.md`)

## Context

In Q2.4 the owner selected "name branches, commits and tasks" with the note: add it only if it is like Cursor's `@`; otherwise not. Cursor's `@` is a way to reference context in a prompt, which is different from auto-naming.

## Decision

Do not implement LLM auto-naming; branch and commit names come from deterministic templates. Implement `@`-mentions (`@file:`, `@kb:`, `@task:`, `@spec:`, `@pack:`, `@session:`, `@url:`) with a fuzzy picker and explicit size budgets. Details in `specs/context-compiler.md` §4.

## Consequences

+ Delivers what the owner meant, aligned with priority #1 (context quality).
- None significant.

## Alternatives considered

Implement both (auto-naming adds cost and latency for little value).
