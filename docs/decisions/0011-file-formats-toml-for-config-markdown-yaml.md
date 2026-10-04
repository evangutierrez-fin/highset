# 0011. File formats: TOML for config, Markdown + YAML frontmatter for content

- Status: Accepted
- Date: 2026-10-03
- Source: questionnaire Q6.3 (`docs/inputs/questionnaire-2026-10-03.md`)

## Context

The owner chose TOML for configuration and Markdown with frontmatter for prompts, profiles and skills (Q6.3). Claude Code agents, Agent Skills (`SKILL.md`) and most tooling use YAML frontmatter.

## Decision

Standalone configuration files are TOML (edited with `toml_edit` to preserve comments). Markdown content files use YAML frontmatter between `---` fences, so `SKILL.md`, `*.agent.md` and `*.prompt.md` stay compatible with the ecosystem.

## Consequences

+ Interoperability with existing skills, agents and prompt files.
- Two syntaxes for users to know; documented with examples.

## Alternatives considered

TOML frontmatter (`+++`): incompatible with the Agent Skills standard and Claude Code files. YAML everywhere: weaker for config files and not idiomatic in Rust.
