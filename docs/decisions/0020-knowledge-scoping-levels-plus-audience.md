# 0020. Knowledge scoping: levels plus audience

- Status: Accepted
- Date: 2026-10-03
- Source: questionnaire Q5.3 (+ note) (`docs/inputs/questionnaire-2026-10-03.md`)

## Context

The owner wants some knowledge to be available only to one AI in one project (Q5.3 note), while most knowledge is shared at global or project level.

## Decision

Every knowledge item has a level (global, workspace, project, local-private) and an optional audience (agents, profiles, tasks; ANDed). Filtering applies at every boundary: compiled files, attachments, on-demand index, MCP and mentions. Committed generated files only include unrestricted, non-local items; restricted items reach their audience via launch-time injection and MCP. Details in `specs/knowledge.md` §2–3.

## Consequences

+ Precise control with simple frontmatter.
- More cases to test; B-003 includes property tests guaranteeing no leakage.

## Alternatives considered

Per-agent folders (duplication). Levels only (cannot target one agent).
