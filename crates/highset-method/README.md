# highset-method

Flows, gates, checks, templates, rituals, and the ADR, changelog and journal writers.

- **Lane:** platform
- **Allowed internal dependencies:** `highset-core`, `highset-store`, `highset-git`
- **Rules:** see [`docs/ARCHITECTURE.md` §2](../../docs/ARCHITECTURE.md#dependency-rules). `cargo run -p xtask -- check-deps` enforces them.
