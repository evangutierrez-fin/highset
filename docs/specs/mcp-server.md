# Spec: HighSet MCP server (`highset-mcp`, daemon `services/context` `mcp.*` methods)

Requirements: CTX-8, CTX-11, AGT-7. ADRs: 0010.

## 1. Shape

- **Command:** `highset mcp serve`, a stdio MCP server built on the official Rust SDK (`rmcp`, pinned; verify the current API in B-009).
- It is a **thin proxy**. It connects to the daemon socket and calls the `mcp.*` methods. It holds no state and touches no files.
- **Identity.** It reads `HIGHSET_SESSION_TOKEN` (set by the daemon at launch) and sends it in `initialize`. The daemon binds the connection to that session: its project, task, agent, profile and audience.
- **Without a token** (an agent started outside HighSet with the server configured globally), it runs in **unscoped read-only mode** for the project discovered from cwd: only `task_get/list`, `spec_get` and `knowledge_search/get`, with audience filtering treating the agent as unknown, so only unrestricted items are visible.
- **Startup** under 50 ms; tool calls add ≤ 10 ms overhead over the daemon call.

## 2. Tools

Names are prefixed `highset_` to avoid collisions. Input schemas are JSON Schema with `additionalProperties: false`. Every output is redacted and audience-filtered.

| Tool | Input | Output | Notes |
|---|---|---|---|
| `highset_task_get` | `{ task?: "T-0042" }` (default: the session's task) | Task fields, body, acceptance criteria, gates, links | |
| `highset_task_list` | `{ status?, label?, limit? }` | Summaries | Session's project only |
| `highset_task_update` | `{ task?, status?: "review", note? }` | Updated task | Agents may only request `review` or add notes (`core-domain.md` §3). Anything else → `unauthorized` with an explanation the agent can read |
| `highset_task_add_note` | `{ task?, text }` | ok | Appends to the task's History section |
| `highset_spec_get` | `{ task? }` | `spec.md`, `plan.md`, `tasks.md` contents, with paths | Read from the task worktree |
| `highset_knowledge_search` | `{ query, kinds?, tags?, limit? (≤ 20) }` | `[{ id, slug, title, kind, snippet, level }]` | Full-text (D-007); before that exists, a SQLite LIKE fallback |
| `highset_knowledge_get` | `{ id_or_slug, max_tokens? }` | Body (truncated with an explicit marker if needed) | |
| `highset_pack_get` | `{ name }` | Pack description and its item list | Only packs active for the session |
| `highset_knowledge_propose` | `{ title, body, kind?, level_hint? }` | proposal id | Goes to the inbox; not visible until approved |
| `highset_memory_propose` | `{ text, type: "decision" \| "learning" \| "todo", evidence? }` | proposal id | Same |
| `highset_decision_propose` | `{ title, context, decision, consequences, alternatives? }` | proposal id | Becomes an ADR when approved |
| `highset_progress_report` | `{ message, percent? }` | ok | Shown in the TUI timeline/status; rate-limited to 1 per 5 s |
| `highset_request_review` | `{ summary }` | ok | Moves the task to `review` (if the flow allows), raises a `gate_pending` attention item |

## 3. Resources

Read-only, URI-addressable:

- `highset://task/{number}`
- `highset://spec/{number}`
- `highset://kb/{slug}`
- `highset://pack/{name}`
- `highset://project`: the project overview, i.e. `PROJECT.md` plus key commands.

## 4. Prompts (optional, if supported by the agent)

- `highset-spec`: "Write the spec for the current task using the template."
- `highset-plan`: "Write the plan and the task breakdown."
- `highset-review`: "Review the current diff against the acceptance criteria."

These mirror the shipped templates (`methodology.md`).

## 5. Security

- Tokens are random (≥ 128 bits). Each one is valid only while its session is running and for that session's project.
- Rate limits: 20 calls/s per session; search results ≤ 20 items; any single response ≤ 256 KiB.
- **Audience filtering** is the main guarantee: an item restricted to `claude-code` must never be returned to a Codex session. B-009 tests this explicitly.

## Verified facts

_(Builder: rmcp version and API notes; MCP spec revision targeted; tested with which agents.)_
