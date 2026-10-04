# highset-plugins

Plugin manifests, install/update/remove, the lockfile, permission review and Claude Code plugin import.

- **Lane:** platform
- **Allowed internal dependencies:** `highset-core`, `highset-store`
- **Rules:** see [`docs/ARCHITECTURE.md` §2](../../docs/ARCHITECTURE.md#dependency-rules). `cargo run -p xtask -- check-deps` enforces them.
