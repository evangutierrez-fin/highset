# 0012. Plugin model: declarative packages now, out-of-process code plugins later

- Status: Accepted
- Date: 2026-10-03
- Source: questionnaire Q7.1–Q7.6 (`docs/inputs/questionnaire-2026-10-03.md`)

## Context

The owner wants plugins (Q7.1) compatible with Claude Code plugins and Agent Skills (Q7.3), installed from git (Q7.4), with declared permissions (Q7.6). The MVP includes a plugin system (Q13.1).

## Decision

v0.1 ships declarative plugins: folders with `highset-plugin.toml` contributing skills, prompts, profiles, packs, convention files, MCP servers, hooks, shell commands and translations. Installed from git or local paths, pinned in `plugins.lock`, with permissions approved at install and on change. Claude Code plugins and marketplaces and standalone skills are imported into the same model. Code plugins (JSON-RPC over stdio) come in v0.2. Details in `specs/plugins.md`.

## Consequences

+ Covers most real needs without a plugin runtime; ecosystem compatibility from day one.
- No isolation for plugin commands in v0.1 (consent-based); documented.

## Alternatives considered

WASM plugins (strong sandbox, but complex and unnecessary for declarative content). Lua scripting (another language to support). Rust dylibs (unstable ABI).
