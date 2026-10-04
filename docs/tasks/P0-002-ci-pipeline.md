---
id: P0-002
title: "CI pipeline"
lane: infra
milestone: M1
size: S
status: todo
depends_on: [P0-001]
requirements: [NFR-PORT-1]
open_questions: []
---

# P0-002 · CI pipeline

**Lane:** infra · **Milestone:** M1 · **Size:** S · **Depends on:** [P0-001](P0-001-bootstrap-the-cargo-workspace.md)

## Goal

Make every push prove formatting, lints, tests, dependency rules and supply-chain policy on macOS and Linux.

## Read first

- [`specs/performance.md#2-ci`](../specs/performance.md#2-ci)
- [`specs/security.md`](../specs/security.md)
- ADR [0017](../decisions/0017-performance-budgets-are-release-blocking.md)
- PRD requirements: NFR-PORT-1

## Scope

- GitHub Actions `ci.yml` on push and PR with jobs: `fmt`, `clippy` (`-D warnings`), `test` (matrix: ubuntu-latest, macos-latest), `deny` (cargo-deny: advisories, licenses, bans, sources), `xtask` (`check-deps`; `check-i18n` once P0-005 lands).
- Cargo caching with a maintained Rust cache action.
- `deny.toml` allowing MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC, Unicode-3.0, Zlib, MPL-2.0, CC0-1.0; deny unknown git sources.
- A placeholder `perf-smoke` job (filled in R-001).
- Developer docs: how to run the same checks locally.

## Acceptance criteria

- [ ] CI is green on `main` for both operating systems.
- [ ] A deliberately mis-formatted commit on a scratch branch fails the `fmt` job (link to the run in the verification log).
- [ ] A full CI run takes ≤ 10 minutes with a warm cache.
- [ ] `cargo deny check` passes locally and in CI.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Notes

The repository is `github.com/evangutierrez-fin/highset` (OQ-3, resolved); use it for badges and links.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
