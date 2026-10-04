# highset-cli

The `highset` binary: the clap command tree and dispatch to the TUI, the daemon, the CLI commands and the MCP server.

- **Lane:** core (skeleton); each lane adds `src/commands/<area>.rs`
- **Allowed internal dependencies:** `highset-core`, `highset-protocol`, `highset-tui`, `highset-daemon`, `highset-mcp`, `highset-i18n`
- **Rules:** see [`docs/ARCHITECTURE.md` §2](../../docs/ARCHITECTURE.md#dependency-rules). `cargo run -p xtask -- check-deps` enforces them.
