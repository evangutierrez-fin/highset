# 0019. Internal LLM tasks through a provider layer with a no-API-key path

- Status: Accepted
- Date: 2026-10-03
- Source: questionnaire Q2.4, Q3.7 (`docs/inputs/questionnaire-2026-10-03.md`)

## Context

The owner selected internal tasks (Q2.4: summaries, memory extraction, tagging, idea→spec) and providers Anthropic, OpenAI and local Ollama/LM Studio (Q3.7). Many users run agents on subscriptions without API keys.

## Decision

`highset-llm` provides a small provider trait with Anthropic (raw HTTP, no official Rust SDK), OpenAI, OpenAI-compatible (local) and an `agent:<kind>` backend that runs an installed agent CLI headless. Tasks are routed per config, run in the background, record usage, and degrade gracefully when no provider is available. Default routes: a fast cheap model for summarize/extract/tag, the default Opus model for idea→spec; all configurable. Details in `specs/llm.md`.

## Consequences

+ Works for API-key users and subscription-only users.
- Model IDs and APIs change; verification at implementation and release.

## Alternatives considered

No internal LLM tasks (loses summaries/memory, pains P2/P6). Hardwire one provider (excludes local and subscription users).
