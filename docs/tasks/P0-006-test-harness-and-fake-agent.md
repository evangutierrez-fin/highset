---
id: P0-006
title: "Test harness and fake agent"
lane: infra
milestone: M1
size: M
status: todo
depends_on: [P0-001]
requirements: [AGT-2]
open_questions: []
---

# P0-006 · Test harness and fake agent

**Lane:** infra · **Milestone:** M1 · **Size:** M · **Depends on:** [P0-001](P0-001-bootstrap-the-cargo-workspace.md)

## Goal

Let every later task test agent behavior deterministically, without real agents or network.

## Read first

- [`specs/agents.md`](../specs/agents.md)
- [`ARCHITECTURE.md#12-testing-strategy`](../ARCHITECTURE.md#12-testing-strategy)
- ADR [0005](../decisions/0005-agent-integration-acp-first-embedded-pty.md)
- PRD requirements: AGT-2

## Scope

- `tools/fake-agent` with three modes driven by a scenario file (YAML or JSON):
  - `--acp`: the agent side of ACP using the pinned `agent-client-protocol` crate (initialize, session/new, session/prompt with scripted `session/update` notifications for message chunks, tool call + update, plan and diff; a scripted permission request; cancellation; end).
  - `--pty`: scripted ANSI output, a prompt, reading input lines, echo, idle periods that look like "needs input", exit code.
  - `--headless`: scripted JSON-lines events modeled on agents' stream-json output, then exit.
  - Optional `canary_secret` in a scenario, printed to test redaction.
- `highset-testkit`: temp `HIGHSET_HOME`, temp git repo with an initial commit, daemon spawner (in-process or child), scenario builders.
- fake-agent README documenting the scenario format with examples; at least three scenarios in `tools/fake-agent/scenarios/`.

## Acceptance criteria

- [ ] One integration test per mode runs the fake agent and asserts the expected output.
- [ ] ACP mode interoperates with the client-side types of the same crate version (in-process round trip).
- [ ] The scenario format is documented and three example scenarios are committed.
- [ ] No test performs network access.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Notes

Verify the current ACP spec and crate API before writing the ACP mode; record the version in specs/agents.md Verified facts.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
