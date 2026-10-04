# xtask

Repository automation: the dependency-rule check, i18n parity, performance scripts and docs generation. Run it with `cargo run -p xtask -- <command>`.

- **Lane:** infra
- **Allowed internal dependencies:** none
- **Rules:** see [`docs/ARCHITECTURE.md` §2](../docs/ARCHITECTURE.md#dependency-rules). `cargo run -p xtask -- check-deps` enforces them.
