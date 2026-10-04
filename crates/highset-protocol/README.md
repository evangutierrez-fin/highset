# highset-protocol

JSON-RPC 2.0 messages, the typed method and event catalog, newline-delimited framing, error codes and the async client shared by every client and the daemon.

- **Lane:** core
- **Allowed internal dependencies:** `highset-core`
- **Rules:** see [`docs/ARCHITECTURE.md` §2](../../docs/ARCHITECTURE.md#dependency-rules). `cargo run -p xtask -- check-deps` enforces them.
