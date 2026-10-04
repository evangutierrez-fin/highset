---
id: P0-001
title: "Bootstrap the Cargo workspace"
lane: infra
milestone: M1
size: S
status: done
depends_on: []
requirements: [NFR-MAINT-1]
open_questions: []
---

# P0-001 · Bootstrap the Cargo workspace

**Lane:** infra · **Milestone:** M1 · **Size:** S · **Depends on:** nothing

## Goal

Create the repository skeleton so every later task has a place to land, with dependency rules enforced from day one.

## Read first

- [`ARCHITECTURE.md#2-crates`](../ARCHITECTURE.md#2-crates)
- [`specs/performance.md`](../specs/performance.md)
- ADR [0002](../decisions/0002-rust-single-binary-cargo-workspace-of-focused.md)
- PRD requirements: NFR-MAINT-1

## Scope

- `git init` with `main` as default branch (if the repo is not yet initialized) and commit the existing docs first.
- Root `Cargo.toml` workspace (edition 2024) with every crate from ARCHITECTURE §2 as a stub: `lib.rs` with a crate-level doc comment and `#![forbid(unsafe_code)]`. `highset-cli` builds the `highset` binary.
- `tools/fake-agent` and `xtask` binary stubs; `highset-testkit` as a dev-only library.
- `rust-toolchain.toml` pinning a current stable toolchain; `rustfmt.toml`; `.editorconfig`; `.gitignore` (target/, local state, `.highset/local/`).
- `[workspace.lints]`: `unsafe_code = "forbid"`; clippy `pedantic` as warn with a short, commented allow-list; every crate inherits.
- `[profile.release]` as in performance.md §3.
- A `README.md` in every crate: purpose, allowed internal dependencies, owning lane.
- `xtask check-deps`: reads `cargo metadata` and fails on any internal dependency edge not allowed by ARCHITECTURE §2.
- `[workspace.package]` sets `license = "MIT"` (ADR 0023) and `repository = "https://github.com/evangutierrez-fin/highset"`; every crate inherits them. All crates stay `publish = false` until R-002 decides on crates.io.

## Out of scope

- CI (P0-002).
- Any product functionality.

## Acceptance criteria

- [ ] `cargo build --workspace` and `cargo clippy --workspace --all-targets -- -D warnings` succeed on macOS and Linux.
- [ ] `cargo run -p highset-cli -- --version` prints `highset <version>`.
- [ ] `cargo run -p xtask -- check-deps` passes, and a unit test with a fake metadata graph proves it fails on a forbidden edge.
- [ ] Every crate directory has a README with purpose, allowed dependencies and lane.
- [ ] The release profile matches performance.md §3.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

Verified on 2026-10-03, macOS 26 (aarch64), toolchain 1.99.0.

- **Build and clippy (macOS):** `cargo build --workspace` and `cargo clippy --workspace --all-targets --all-features -- -D warnings` finish with no warnings. **Linux:** no local container runtime is available, so the Linux link step is proven by the first green CI run of P0-002 on ubuntu-latest. P0-002 has an acceptance criterion to link that run here. Interim evidence: `cargo clippy --workspace --all-targets --target x86_64-unknown-linux-gnu -- -D warnings` is clean (checks and lints the Linux code paths without linking).
- **`--version`:** `cargo run -p highset-cli -- --version` prints `highset 0.1.0`. Unit test `highset_cli::tests::version_flag_prints_name_and_version`.
- **check-deps:** `cargo run -p xtask -- check-deps` prints `check-deps: all internal dependency edges are allowed (<workspace root>)`. Unit tests in `xtask/src/check_deps.rs` use fake metadata graphs: `forbidden_edge_fails` (tui → store), `core_cannot_depend_on_internal_crates`, `sideways_service_edges_fail_even_as_build_dependencies`, `testkit_only_as_dev_dependency`, `dev_dependencies_follow_the_same_rules`, `unknown_member_fails`, `allowed_graph_passes`, `every_rule_names_a_known_crate`.
- **Crate READMEs:** all 19 crate directories (`crates/*`, `tools/fake-agent`, `xtask`) have a `README.md` with purpose, lane and allowed internal dependencies.
- **Release profile:** `[profile.release]` has `lto = "thin"`, `codegen-units = 1`, `strip = true`, `panic = "unwind"` (performance.md §3 rule 9). Stub release binary: 593 KB (592,976 bytes, with clap `suggestions`).
- **DoD:** fmt clean; clippy clean; `cargo test --workspace` green (10 tests); no `unwrap`/`expect`/`panic!` in non-test code (now enforced by `clippy::unwrap_used`, `expect_used`, `panic` at workspace level, with tests exempt via `clippy.toml`); every crate has a crate-level doc comment and `missing_docs` is on; no user-facing strings besides the CLI `about` text, which becomes a Fluent key in P0-005; no secrets.
- **Docs:** ARCHITECTURE.md §2 gained rules 5–6 that make implicit edges explicit (`highset-core` usable by every crate; `highset-cli` → `highset-protocol`; `highset-testkit` dev-only).
- **Review:** reviewer subagent verdict PASS (2026-10-03) after fixing one blocking item (Linux evidence tracked in P0-002) and one should-fix (check-deps resolved the workspace root at compile time, which caused a false pass with a shared `CARGO_TARGET_DIR`).
- **CHANGELOG:** `CHANGELOG.md` is created in P0-003, with the `--version` entry.
- **Linux (closed by P0-002, 2026-10-04):** first CI run on ubuntu-latest, all green: build `--all-targets`, clippy `-D warnings`, test, check-deps: <https://github.com/evangutierrez-fin/highset/actions/runs/37164592022>.
