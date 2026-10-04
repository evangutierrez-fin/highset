---
id: P0-004
title: "Paths and layered configuration"
lane: infra
milestone: M1
size: M
status: todo
depends_on: [P0-001]
requirements: [HRN-4, DATA-4]
open_questions: []
---

# P0-004 · Paths and layered configuration

**Lane:** infra · **Milestone:** M1 · **Size:** M · **Depends on:** [P0-001](P0-001-bootstrap-the-cargo-workspace.md)

## Goal

One correct implementation of where things live and how configuration layers merge, used by every other crate.

## Read first

- [`specs/core-domain.md#6-configuration-model`](../specs/core-domain.md#6-configuration-model)
- [`specs/core-domain.md#7-paths-highset_corepaths`](../specs/core-domain.md#7-paths-highset_corepaths)
- [`specs/storage.md#21-configtoml-global-workspacetoml-projecttoml`](../specs/storage.md#21-configtoml-global-workspacetoml-projecttoml)
- ADR [0008](../decisions/0008-storage-locations-content-vs-state-global-vs.md)
- ADR [0011](../decisions/0011-file-formats-toml-for-config-markdown-yaml.md)
- PRD requirements: HRN-4, DATA-4

## Scope

- `highset_core::paths` per core-domain §7, including env overrides and the rule that `state_dir` must not be inside `content_dir`.
- `highset_core::config`: typed `Config` plus `ConfigLayer`; merge global → workspace → project → task with the rules in core-domain §6 (including `+key` list extension) and origin tracking per value.
- Load with `toml_edit` (keeps comments for later writes) and report precise error locations (file:line:col).
- `highset config show [--resolved] [--task T-n]` in the CLI (local for now; it goes through the daemon after P1-005).

## Acceptance criteria

- [ ] Unit tests cover the precedence of all four layers, `+key` extension, scalar/array replacement, and unknown keys producing warnings rather than errors.
- [ ] `content_dir` can be relocated by config and by env var; a `state_dir` inside `content_dir` is rejected with a clear error.
- [ ] An invalid TOML file reports `path:line:col` and a hint.
- [ ] `config show --resolved` annotates each value with its origin layer and file.
- [ ] Paths with spaces and non-ASCII characters work (tested).
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
