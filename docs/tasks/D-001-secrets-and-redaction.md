---
id: D-001
title: "Secrets and redaction"
lane: platform
milestone: M2
size: M
status: todo
depends_on: [P1-005]
requirements: [DATA-2, DATA-3]
open_questions: []
---

# D-001 · Secrets and redaction

**Lane:** platform · **Milestone:** M2 · **Size:** M · **Depends on:** [P1-005](P1-005-daemon-skeleton-and-client-autostart.md)

## Goal

Secrets never leak into files, transcripts, logs or generated configs.

## Read first

- [`specs/security.md#2-secrets`](../specs/security.md#2-secrets)
- [`specs/security.md#3-redaction-data-3`](../specs/security.md#3-redaction-data-3)
- ADR [0016](../decisions/0016-secrets-in-the-os-keychain-redact-before.md)
- PRD requirements: DATA-2, DATA-3

## Scope

- Keychain backend via `keyring` (mock backend for CI); CLI `highset secret set|list|rm` (value from a hidden prompt or stdin only).
- `secret:` resolution API used by launches, LLM calls and MCP registry entries.
- `Redactor` with exact known values (Aho-Corasick), built-in patterns, optional entropy detector; a shared trait applied by stores and outputs; log field redaction.

## Acceptance criteria

- [ ] Positive and negative tests for every built-in pattern.
- [ ] Exact stored values are redacted even when they match no pattern.
- [ ] No RPC method or CLI command returns a secret value (test enumerating methods).
- [ ] Redaction ≤ 1 ms per 64 KiB (bench).
- [ ] Linux CI uses the mock backend; manual macOS Keychain check documented.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
