# 0018. Follow industry standards before inventing formats

- Status: Accepted
- Date: 2026-10-03
- Source: questionnaire Q13.10, Q7.3, Q8.7 (`docs/inputs/questionnaire-2026-10-03.md`)

## Context

The owner requires integration with what professionals use today and adherence to standard methodology (Q13.10).

## Decision

Prefer existing standards and conventions: AGENTS.md (agent instructions), Agent Skills `SKILL.md`, MCP (tools), ACP (agent control), GitHub Spec Kit layout (specs), VS Code/Copilot-style `*.instructions.md` / `*.prompt.md` / `*.agent.md` naming, MADR (ADRs), Conventional Commits, SemVer, Keep a Changelog, GitHub Flow with PRs, TOML config. A new HighSet-specific format requires an ADR explaining why no standard fits.

## Consequences

+ Users and agents recognize the formats; files work outside HighSet.
- Standards evolve; adapters and templates must be re-verified periodically (the "Verified facts" sections).

## Alternatives considered

Custom formats optimized for HighSet (lock-in, learning curve).
