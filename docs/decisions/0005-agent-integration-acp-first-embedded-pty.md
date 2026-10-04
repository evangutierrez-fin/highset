# 0005. Agent integration: ACP first, embedded PTY fallback, headless for automation

- Status: Accepted
- Date: 2026-10-03
- Source: questionnaire Q3.2, Q9.4 (`docs/inputs/questionnaire-2026-10-03.md`)

## Context

HighSet needs to understand what agents do (messages, tool calls, diffs, permission requests), not only show text. ACP (Agent Client Protocol, created by Zed) standardizes editor↔agent communication and has an official Rust crate. Not every agent supports ACP equally; every agent has a terminal UI.

## Decision

HighSet is an ACP client. Each adapter prefers ACP when available, falls back to an embedded PTY that shows the agent's real terminal UI, and supports headless JSON streaming for automation. Agent-specific facts (flags, ACP adapters, config paths) are hypotheses until verified in each adapter task and recorded in `specs/agents.md`.

## Consequences

+ Structured view, centralized permission handling, and precise attention signals where ACP exists.
+ Every agent works on day one through PTY.
- Three modes to maintain per adapter; fixture tests and a fake agent keep this manageable.

## Alternatives considered

PTY only (no structure, poor attention detection). Headless only (no interactive work). Wrapping agent SDKs directly (language mismatch, vendor lock-in).
