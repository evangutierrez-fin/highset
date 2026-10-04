# Architecture decision records

Binding for builder agents (see `AGENTS.md`). Format: short MADR (`0000-template.md`).

| # | Decision | Status |
|---|---|---|
| [0001](0001-record-architecture-decisions-with-madr.md) | Record architecture decisions with MADR | Accepted |
| [0002](0002-rust-single-binary-cargo-workspace-of-focused.md) | Rust, single binary, Cargo workspace of focused crates | Accepted |
| [0003](0003-orchestrate-existing-agents-instead-of-building.md) | Orchestrate existing agents instead of building an agent | Accepted |
| [0004](0004-local-daemon-with-thin-clients-over-a-unix.md) | Local daemon with thin clients over a Unix socket (JSON-RPC 2.0) | Accepted |
| [0005](0005-agent-integration-acp-first-embedded-pty.md) | Agent integration: ACP first, embedded PTY fallback, headless for automation | Accepted |
| [0006](0006-one-git-worktree-and-branch-per-task.md) | One git worktree and branch per task | Accepted |
| [0007](0007-plain-files-are-the-source-of-truth-sqlite-and.md) | Plain files are the source of truth; SQLite and tantivy are caches | Accepted |
| [0008](0008-storage-locations-content-vs-state-global-vs.md) | Storage locations: content vs. state, global vs. project vs. local | Accepted |
| [0009](0009-highset-is-the-source-of-truth-for-harness-and.md) | HighSet is the source of truth for harness and context; compile and sync to each agent | Accepted |
| [0010](0010-highset-exposes-its-own-mcp-server-to-agents.md) | HighSet exposes its own MCP server to agents | Accepted |
| [0011](0011-file-formats-toml-for-config-markdown-yaml.md) | File formats: TOML for config, Markdown + YAML frontmatter for content | Accepted |
| [0012](0012-plugin-model-declarative-packages-now-out-of.md) | Plugin model: declarative packages now, out-of-process code plugins later | Accepted |
| [0013](0013-methodology-adaptive-spec-driven-development.md) | Methodology: adaptive spec-driven development with Spec Kit–compatible artifacts | Accepted |
| [0014](0014-terminal-ui-ratatui-ide-layout-hybrid-keymap.md) | Terminal UI: ratatui, IDE layout, hybrid keymap, Fluent i18n | Accepted |
| [0015](0015-search-tantivy-full-text-and-nucleo-fuzzy-no.md) | Search: tantivy full-text and nucleo fuzzy; no embeddings in v0.1 | Accepted |
| [0016](0016-secrets-in-the-os-keychain-redact-before.md) | Secrets in the OS keychain; redact before persisting | Accepted |
| [0017](0017-performance-budgets-are-release-blocking.md) | Performance budgets are release-blocking | Accepted |
| [0018](0018-follow-industry-standards-before-inventing.md) | Follow industry standards before inventing formats | Accepted |
| [0019](0019-internal-llm-tasks-through-a-provider-layer-with.md) | Internal LLM tasks through a provider layer with a no-API-key path | Accepted |
| [0020](0020-knowledge-scoping-levels-plus-audience.md) | Knowledge scoping: levels plus audience | Accepted |
| [0021](0021-context-mentions-instead-of-auto-naming.md) | Context mentions (@) instead of auto-naming | Accepted |
| [0022](0022-local-first-sync-ready-layout-sync-providers.md) | Local-first, sync-ready layout; sync providers deferred | Accepted |
| [0023](0023-license.md) | License: MIT | Accepted |
