---
id: D-002
title: "LLM provider layer"
lane: platform
milestone: M3
size: M
status: todo
depends_on: [D-001]
requirements: [LLM-1]
open_questions: []
---

# D-002 · LLM provider layer

**Lane:** platform · **Milestone:** M3 · **Size:** M · **Depends on:** [D-001](D-001-secrets-and-redaction.md)

## Goal

Small model calls for internal tasks, with or without API keys.

## Read first

- [`specs/llm.md#1-providers`](../specs/llm.md#1-providers)
- [`specs/llm.md#2-routing`](../specs/llm.md#2-routing)
- ADR [0019](../decisions/0019-internal-llm-tasks-through-a-provider-layer-with.md)
- PRD requirements: LLM-1

## Scope

- Provider trait; Anthropic over raw HTTPS per the current Messages API docs (structured outputs for JSON, `stop_reason` checks including `refusal`, streaming for long outputs); OpenAI (verify the current endpoint); OpenAI-compatible (local servers); `agent:<kind>` headless backend.
- Routing per task with fallback; retries (429/5xx); timeouts; usage recording.
- Record API versions and model IDs in Verified facts.

## Acceptance criteria

- [ ] wiremock tests per provider: success, 429 retry, 5xx, refusal and other stop reasons, timeout.
- [ ] The agent-headless backend is tested with fake-agent `--headless`.
- [ ] Missing credentials produce an actionable, localized error; no network call happens unless a task runs.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
