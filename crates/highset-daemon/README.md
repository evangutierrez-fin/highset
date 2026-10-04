# highset-daemon

Wiring: the socket server, service registry, event bus, session supervisor, attention, hooks engine, notifications, scheduler and backups.

- **Lane:** core (skeleton) + every lane in `src/services/<area>/`
- **Allowed internal dependencies:** `highset-core`, `highset-protocol`, `highset-store`, `highset-search`, `highset-git`, `highset-agents`, `highset-context`, `highset-llm`, `highset-method`, `highset-plugins`, `highset-secrets`
- **Rules:** see [`docs/ARCHITECTURE.md` §2](../../docs/ARCHITECTURE.md#dependency-rules). `cargo run -p xtask -- check-deps` enforces them.
