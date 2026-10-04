# highset-llm

LLM provider clients (Anthropic, OpenAI, OpenAI-compatible, agent-headless) and the prompts for internal tasks.

- **Lane:** platform
- **Allowed internal dependencies:** `highset-core`, `highset-secrets`
- **Rules:** see [`docs/ARCHITECTURE.md` §2](../../docs/ARCHITECTURE.md#dependency-rules). `cargo run -p xtask -- check-deps` enforces them.
