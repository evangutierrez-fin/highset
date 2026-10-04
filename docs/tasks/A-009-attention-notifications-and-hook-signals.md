---
id: A-009
title: "Attention, notifications and hook signals"
lane: agents
milestone: M2
size: M
status: todo
depends_on: [A-002]
requirements: [AGT-6, AGT-9, UX-6]
open_questions: []
---

# A-009 · Attention, notifications and hook signals

**Lane:** agents · **Milestone:** M2 · **Size:** M · **Depends on:** [A-002](A-002-agent-adapter-framework-and-session-supervisor.md)

## Goal

Never miss an agent that is waiting.

## Read first

- [`specs/agents.md#7-attention-and-notifications-servicesattention`](../specs/agents.md#7-attention-and-notifications-servicesattention)
- ADR [0005](../decisions/0005-agent-integration-acp-first-embedded-pty.md)
- PRD requirements: AGT-6, AGT-9, UX-6

## Scope

- Attention service: kinds, urgency ordering, resolution; counts added to `ui.bootstrap`.
- OS notifications through `notify-rust`, rate-limited, toggled per kind via `[notifications]`.
- CLI `highset attention list|approve|deny`.
- `highset hook signal --kind needs_input|finished|permission` plus the `hook.signal` method, authenticated with the session token.

## Acceptance criteria

- [ ] A fake ACP permission request creates an attention item; `attention approve` resolves it and the agent receives the outcome.
- [ ] `hook signal` with a valid token raises an item; an invalid token is rejected.
- [ ] Notifications are rate-limited (≤ 1 per session per 10 s) and respect toggles (unit tests with a mock notifier).
- [ ] Real notifications verified manually on macOS and Linux (documented).
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
