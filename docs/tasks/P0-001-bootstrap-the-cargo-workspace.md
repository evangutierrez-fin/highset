---
id: P0-001
title: "Bootstrap the Cargo workspace"
lane: infra
milestone: M1
size: S
status: todo
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

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
