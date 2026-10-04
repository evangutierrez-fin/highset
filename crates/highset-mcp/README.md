# highset-mcp

`highset mcp serve`: the stdio MCP server that proxies agent requests to the daemon, authenticated with a per-session token.

- **Lane:** context
- **Allowed internal dependencies:** `highset-core`, `highset-protocol`
- **Rules:** see [`docs/ARCHITECTURE.md` §2](../../docs/ARCHITECTURE.md#dependency-rules). `cargo run -p xtask -- check-deps` enforces them.
