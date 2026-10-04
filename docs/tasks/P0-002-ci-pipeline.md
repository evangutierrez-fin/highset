---
id: P0-002
title: "CI pipeline"
lane: infra
milestone: M1
size: S
status: in-progress
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
- [ ] The first green CI run on `ubuntu-latest` (build, clippy `-D warnings`, test, check-deps) is linked in P0-001's verification log. This closes P0-001's Linux criterion.
- [ ] A deliberately mis-formatted commit on a scratch branch fails the `fmt` job (link to the run in the verification log).
- [ ] A full CI run takes ≤ 10 minutes with a warm cache.
- [ ] `cargo deny check` passes locally and in CI.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Notes

The repository is `github.com/evangutierrez-fin/highset` (OQ-3, resolved); use it for badges and links.

## Verification log

Verified on 2026-10-04 (UTC). Repository created public at <https://github.com/evangutierrez-fin/highset> (OQ-4).

- **CI green on both operating systems:** run [37164592022](https://github.com/evangutierrez-fin/highset/actions/runs/37164592022) on the task branch: `fmt`, `clippy`, `test (ubuntu-latest)`, `test (macos-latest)`, `deny`, `xtask`, `perf-smoke` all succeeded. The run on `main` after the merge is linked below.
- **Mis-formatted commit fails `fmt`:** scratch branch `task/P0-002-scratch-fmt-fail` (deleted afterwards, never merged) added `fn   badly_formatted( ) {}`. Run [37164604855](https://github.com/evangutierrez-fin/highset/actions/runs/37164604855): `fmt` failed (`cargo fmt --all -- --check` exit 1). `clippy` and `test` also failed there because the function is unused and warnings are denied, which is the intended policy.
- **Full run ≤ 10 minutes with a warm cache:** the first run, with a cold cache, took 51 s end to end (00:19:28 → 00:20:19 UTC). The slowest job was `test (macos-latest)` at 37 s. Warm runs can only be faster; this will grow with the codebase, and R-001 re-measures it.
- **`cargo deny check` passes locally and in CI:** local `cargo-deny 0.20.2` prints `advisories ok, bans ok, licenses ok, sources ok`. The CI `deny` job uses `EmbarkStudios/cargo-deny-action@v2` and passed.
- **P0-001 Linux criterion:** linked in P0-001's verification log (the same run 37164592022).
- **DoD:** fmt, clippy and tests are green locally and in CI; no Rust code changed; no secrets (the workflow has `permissions: contents: read` and no secrets); `docs/development.md` documents the local equivalents of every job.

Verified facts (action versions on 2026-10-03, from the GitHub releases API): `actions/checkout` v7.0.1, `actions-rust-lang/setup-rust-toolchain` v2.0.0 (reads `rust-toolchain.toml`, wraps `Swatinem/rust-cache`, `rustflags` default is empty in v2), `EmbarkStudios/cargo-deny-action` v2.1.1. The v2 toolchain action also makes cargo deny warnings in builds (the `build.warnings` setting), as the scratch run shows.
