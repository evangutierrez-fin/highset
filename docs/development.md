# Development

How to build HighSet and run the same checks as CI. Contribution rules are in [`../CONTRIBUTING.md`](../CONTRIBUTING.md) and [`../AGENTS.md`](../AGENTS.md).

## Requirements

- macOS 13+ or Linux (glibc). Windows is not supported.
- [rustup](https://rustup.rs). The toolchain is pinned in `rust-toolchain.toml` and installs itself on the first `cargo` command.
- [`cargo-deny`](https://github.com/EmbarkStudios/cargo-deny) for supply-chain checks: `cargo install cargo-deny --locked` or `brew install cargo-deny`.

## Run the CI checks locally

CI (`.github/workflows/ci.yml`) runs these jobs on every push and pull request. Each line below is what the job runs:

| Job | Command | Runs on |
|---|---|---|
| `fmt` | `cargo fmt --all -- --check` | Linux |
| `clippy` | `cargo clippy --workspace --all-targets --all-features --locked -- -D warnings` | Linux |
| `test` | `cargo build --workspace --all-targets --locked` then `cargo test --workspace --locked` | Linux and macOS |
| `deny` | `cargo deny check advisories licenses bans sources` | Linux |
| `xtask` | `cargo run -p xtask --locked -- check-deps` (and `check-i18n` once it exists) | Linux |
| `perf-smoke` | placeholder until R-001 | Linux |

Before you push, run them all:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo deny check
cargo run -p xtask -- check-deps
```

Always pass `--workspace`: the workspace's default member is the CLI, so a plain `cargo test` only tests that one crate.

To lint the Linux code paths from a Mac without a container: `rustup target add x86_64-unknown-linux-gnu`, then `cargo clippy --workspace --all-targets --target x86_64-unknown-linux-gnu -- -D warnings`. It checks and lints but does not link.

## Policies enforced by CI

- **Formatting and lints.** `rustfmt.toml` and `[workspace.lints]` in `Cargo.toml`. `unwrap`, `expect` and `panic!` are lint errors outside tests (`clippy.toml` exempts tests).
- **Dependency rules.** `xtask check-deps` enforces the crate graph in [`ARCHITECTURE.md` §2](ARCHITECTURE.md#dependency-rules).
- **Supply chain.** `deny.toml`: RustSec advisories fail the build; licenses must be on the allow-list (compatible with MIT); crates come only from crates.io, never from unknown registries or git.
- **Locked dependencies.** CI uses `--locked`, so `Cargo.lock` must be committed and up to date.
