# highset-store

Content files (Markdown/TOML read and write, atomic and round-trip safe), the SQLite cache and its migrations, the file watcher, and backups.

- **Lane:** core
- **Allowed internal dependencies:** `highset-core`
- **Rules:** see [`docs/ARCHITECTURE.md` §2](../../docs/ARCHITECTURE.md#dependency-rules). `cargo run -p xtask -- check-deps` enforces them.
