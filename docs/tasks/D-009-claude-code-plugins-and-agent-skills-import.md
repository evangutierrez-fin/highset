---
id: D-009
title: "Claude Code plugins and Agent Skills import"
lane: platform
milestone: M4
size: M
status: todo
depends_on: [D-008]
requirements: [PLG-4, PLG-5]
open_questions: []
---

# D-009 · Claude Code plugins and Agent Skills import

**Lane:** platform · **Milestone:** M4 · **Size:** M · **Depends on:** [D-008](D-008-plugins-v1-declarative-packages.md)

## Goal

Start with the existing ecosystem instead of an empty one.

## Read first

- [`specs/plugins.md#5-claude-code-compatibility-plg-4-plg-5`](../specs/plugins.md#5-claude-code-compatibility-plg-4-plg-5)
- ADR [0012](../decisions/0012-plugin-model-declarative-packages-now-out-of.md)
- ADR [0018](../decisions/0018-follow-industry-standards-before-inventing.md)
- PRD requirements: PLG-4, PLG-5

## Scope

- Spike: verify the current plugin.json, marketplace.json, hooks and SKILL.md formats.
- Import Claude Code plugins and marketplaces with the mapping table; hooks are never auto-enabled; standalone skill folders.

## Acceptance criteria

- [ ] A fixture Claude Code plugin and marketplace import correctly; unsupported fields are reported, never silently dropped.
- [ ] Imported skills appear in the Claude Code sync output and as on-demand knowledge for Codex (golden).
- [ ] Verified facts recorded.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
