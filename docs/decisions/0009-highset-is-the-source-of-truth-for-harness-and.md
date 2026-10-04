# 0009. HighSet is the source of truth for harness and context; compile and sync to each agent

- Status: Accepted
- Date: 2026-10-03
- Source: questionnaire Q5.4, Q6.2 (`docs/inputs/questionnaire-2026-10-03.md`)

## Context

Configuring each agent separately is pain P3. The owner chose HighSet as the source of truth that syncs every CLI (Q6.2).

## Decision

Profiles, packs, convention files and the MCP registry are compiled into an agent-neutral `ContextBundle`, then emitted per agent (files with managed blocks and owned keys, or launch-time flags/env). Emitters never clobber user content, are idempotent, support `--dry-run` diffs, and never write secret values. Details in `specs/context-compiler.md`.

## Consequences

+ Define once, works with every agent; switching agents is cheap.
- Must track each agent's config format (verification steps per target).
- Lossy mappings for agents with coarse permission models; surfaced in `context explain`.

## Alternatives considered

Read-only viewer of each CLI's config (does not solve P3). Agent-specific profiles only (duplication).
