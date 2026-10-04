# fake-agent

A test double for agent CLIs. It speaks ACP, PTY and headless JSON according to scripted scenarios, so tests never call real agents.

- **Lane:** infra
- **Allowed internal dependencies:** `highset-core`
- **Rules:** see [`docs/ARCHITECTURE.md` §2](../../docs/ARCHITECTURE.md#dependency-rules). `cargo run -p xtask -- check-deps` enforces them.
