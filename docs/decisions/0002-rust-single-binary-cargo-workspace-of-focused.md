# 0002. Rust, single binary, Cargo workspace of focused crates

- Status: Accepted
- Date: 2026-10-03
- Source: questionnaire Q2.5, Q12.3, Q12.4, Q13.9 (`docs/inputs/questionnaire-2026-10-03.md`)

## Context

The owner asked for a Rust terminal app (Q2.5), a Cargo workspace with separate crates (Q12.3) and the stack listed in Q12.4. Performance and low resource use are the owner's deal-breakers (Q13.9). Several agents will build lanes in parallel.

## Decision

One binary named `highset` built from a Cargo workspace. Crates and dependency rules as in `ARCHITECTURE.md` §2, enforced by `xtask check-deps`. Rust stable pinned via `rust-toolchain.toml`, edition 2024, `#![forbid(unsafe_code)]` everywhere. Base stack: tokio, ratatui + crossterm, clap, serde + toml, rusqlite, tantivy, nucleo, portable-pty + vt100, rmcp, agent-client-protocol, keyring, tracing, git CLI.

## Consequences

+ Clear ownership boundaries let lanes work in parallel with few conflicts.
+ Incremental compilation stays fast.
+ One binary keeps installation and upgrades trivial.
- More crates means more boilerplate (READMEs, Cargo.toml); accepted.

## Alternatives considered

Single crate: simpler but conflicts between parallel agents and slower builds. Multiple binaries (`highsetd`, `highset`): harder installs and version skew.
