# 0016. Secrets in the OS keychain; redact before persisting

- Status: Accepted
- Date: 2026-10-03
- Source: questionnaire Q10.2, Q10.3 (`docs/inputs/questionnaire-2026-10-03.md`)

## Context

Agents read files and transcripts are reused as context. A leaked key propagates (Q10.3).

## Decision

Secrets are stored only in the OS keychain (`keyring`), referenced as `secret:<name>`, resolved inside the daemon at launch and injected as env vars; no API returns secret values and no generated file contains them. A redaction engine (exact known values + patterns) runs before anything derived from agent output is persisted or returned through MCP. Details in `specs/security.md`.

## Consequences

+ Strong default against the most likely leak paths.
- Linux servers without Secret Service need a documented fallback.

## Alternatives considered

`.env` files (easy to leak to agents and git). 1Password CLI only (extra dependency; can be added as a backend later).
