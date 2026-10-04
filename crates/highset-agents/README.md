# highset-agents

The agent adapter trait and registry, the ACP client, the PTY runner, headless runners, the adapters for Claude Code, Codex, opencode and Cursor CLI, and transcript harvesters.

- **Lane:** agents
- **Allowed internal dependencies:** `highset-core`, `highset-git`
- **Rules:** see [`docs/ARCHITECTURE.md` §2](../../docs/ARCHITECTURE.md#dependency-rules). `cargo run -p xtask -- check-deps` enforces them.
