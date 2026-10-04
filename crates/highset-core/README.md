# highset-core

Domain types and IDs, the task status machine, events, flow types, the `ContextBundle` IR, the config model and `paths`. No async and no IO except path resolution. This crate is a contract: it freezes at milestone M1.

- **Lane:** core
- **Allowed internal dependencies:** none
- **Rules:** see [`docs/ARCHITECTURE.md` §2](../../docs/ARCHITECTURE.md#dependency-rules). `cargo run -p xtask -- check-deps` enforces them.
