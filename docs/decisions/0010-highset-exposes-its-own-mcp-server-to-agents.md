# 0010. HighSet exposes its own MCP server to agents

- Status: Accepted
- Date: 2026-10-03
- Source: questionnaire Q3.5, Q5.4 (`docs/inputs/questionnaire-2026-10-03.md`)

## Context

Agents need to fetch context on demand and report progress without the owner copying text around. All target agents speak MCP.

## Decision

`highset mcp serve` is a stdio MCP server (rmcp) that proxies to the daemon, authenticated with a per-session token. It exposes task, spec, knowledge and proposal tools and resources, with audience filtering. It is auto-registered in every sync target. Details in `specs/mcp-server.md`.

## Consequences

+ Agent-agnostic on-demand context; agents become participants in task tracking.
- Agents could be manipulated into calling tools; mitigated by actor-aware transitions and proposal approval.

## Alternatives considered

Shared files only (handoff.md): unstructured, stale. Copy/paste: the problem itself.
