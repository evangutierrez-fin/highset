# 0003. Orchestrate existing agents instead of building an agent

- Status: Accepted
- Date: 2026-10-03
- Source: questionnaire Q2.3, Q3.1 (`docs/inputs/questionnaire-2026-10-03.md`)

## Context

Claude Code, Codex CLI, opencode and Cursor CLI are improving quickly. The owner's pains (context, organization, method, coordination) are about what surrounds the agents, not their inner loop.

## Decision

HighSet is an orchestrator. It launches and observes external agent CLIs and gives them context, harness configuration and method. Direct model calls are limited to small internal tasks (ADR-0019).

## Consequences

+ HighSet benefits automatically from every agent improvement.
+ Much smaller scope than an agent.
- Dependent on third-party CLIs and their changes (mitigated by isolated adapters and verification steps, ADR-0005).

## Alternatives considered

Own agent loop (rejected: huge scope, competes with best-in-class tools). Hybrid with a full own agent (deferred: may revisit after v0.2).
