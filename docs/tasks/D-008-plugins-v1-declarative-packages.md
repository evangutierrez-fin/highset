---
id: D-008
title: "Plugins v1: declarative packages"
lane: platform
milestone: M4
size: L
status: todo
depends_on: [B-004, B-005]
requirements: [PLG-1, PLG-2, PLG-3]
open_questions: []
---

# D-008 · Plugins v1: declarative packages

**Lane:** platform · **Milestone:** M4 · **Size:** L · **Depends on:** [B-004](B-004-context-packs.md), [B-005](B-005-harness-profiles-and-mcp-server-registry.md)

## Goal

Extend HighSet with shareable packages, with explicit consent.

## Read first

- [`specs/plugins.md#1-package-layout-declarative-v01`](../specs/plugins.md#1-package-layout-declarative-v01)
- [`specs/plugins.md#2-manifest`](../specs/plugins.md#2-manifest)
- [`specs/plugins.md#3-sources-and-lockfile-plg-2`](../specs/plugins.md#3-sources-and-lockfile-plg-2)
- [`specs/plugins.md#4-contribution-model`](../specs/plugins.md#4-contribution-model)
- ADR [0012](../decisions/0012-plugin-model-declarative-packages-now-out-of.md)
- PRD requirements: PLG-1, PLG-2, PLG-3

## Scope

- Manifest; sources (github/git/path); clone at a pinned commit; `plugins.lock`; `plugin inspect|add|update|rm`.
- Permission review and hash; contributions registered as the `plugin:<id>` level.
- Enforcement: hooks and commands may only use declared executables and secrets.
- An example plugin in `examples/plugins/hello-pack`.

## Acceptance criteria

- [ ] The example plugin installs, and its pack/profile/skill appear in compilation (golden).
- [ ] Changed permissions on update require re-approval (test).
- [ ] Uninstalling leaves no residue (test comparing the content tree).
- [ ] A hook calling an undeclared executable is refused (test).
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
