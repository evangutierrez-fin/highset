# highset-testkit

Test helpers: temporary `HIGHSET_HOME` and git repos, a daemon spawner and scenario builders. Used only as a dev-dependency.

- **Lane:** infra
- **Allowed internal dependencies:** any workspace crate (dev-only: other crates may use it only as a dev-dependency)
- **Rules:** see [`docs/ARCHITECTURE.md` §2](../../docs/ARCHITECTURE.md#dependency-rules). `cargo run -p xtask -- check-deps` enforces them.
