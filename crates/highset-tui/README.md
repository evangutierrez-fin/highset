# highset-tui

The ratatui terminal application. It reaches state only through the daemon.

- **Lane:** tui
- **Allowed internal dependencies:** `highset-core`, `highset-protocol`, `highset-i18n`
- **Rules:** see [`docs/ARCHITECTURE.md` §2](../../docs/ARCHITECTURE.md#dependency-rules). `cargo run -p xtask -- check-deps` enforces them.
