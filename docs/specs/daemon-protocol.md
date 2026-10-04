# Spec: daemon and protocol (`highset-daemon`, `highset-protocol`)

Requirements: AGT-4, INT-2, UX-10, PERF-3/4/5. ADRs: 0004.

## 1. Process model

- One daemon per user. It is started by `highset daemon run` (foreground) or `highset daemon start` (detached). Every client auto-starts it when the socket is missing.
- **Auto-start:** spawn `highset daemon run --detached` with stdio closed and a new session (`setsid`), then wait for the socket for up to 1 s (poll with backoff, 10 ms → 100 ms) before failing with a clear error.
- **Single instance:** an exclusive lock on `state/run/daemon.lock` (via the `fs4` or `fd-lock` crate). A second instance exits with code 3 and the message "already running (pid N)".
- **Stale socket:** if the lock is free and the socket file exists, delete the socket and start.
- **Shutdown:** SIGTERM/SIGINT or `daemon.shutdown`. The daemon stops accepting connections, asks sessions to stop (configurable: `on_shutdown = "detach" | "stop"`, default `stop` with a 5 s grace period), flushes the store and search, removes the socket.
- **Upgrade safety:** clients send their version in the handshake. If the major protocol version differs, the client tells the user to run `highset daemon restart`.

## 2. Transport

- Unix domain socket at `state/run/highset.sock`, created with mode `0600`; the `run/` directory is `0700`.
- Frames: UTF-8 JSON, one message per line (`\n`). Maximum frame size 8 MiB; larger payloads must be paged.
- JSON-RPC 2.0: requests (`id`), responses, notifications (no `id`). Batches are not supported.
- **Handshake (first request):**

  ```json
  {"jsonrpc":"2.0","id":1,"method":"initialize","params":{"client":"tui","client_version":"0.1.0","protocol":1,"session_token":null}}
  ```

  The response contains `daemon_version`, `protocol`, `capabilities` and `server_time`.
- The MCP shim sends `session_token` (from `HIGHSET_SESSION_TOKEN`). The daemon binds the connection to that session's identity and audience, and limits methods to the `mcp.*` subset.

## 3. Errors

JSON-RPC error codes:

- `-32700`…`-32603`: standard JSON-RPC errors.
- `1000+`: HighSet errors, mapped in `highset-protocol::error`:

| Code | Name | Example |
|---|---|---|
| 1000 | `not_found` | task T-0099 not found |
| 1001 | `invalid_transition` | inbox → done not allowed by flow `feature` |
| 1002 | `gate_required` | approve_plan pending (pass `skip_reason` to skip in guide mode) |
| 1003 | `conflict` | worktree dirty; pass `force` |
| 1004 | `validation` | invalid frontmatter at path:line |
| 1005 | `unauthorized` | MCP token invalid or method not allowed for an agent |
| 1006 | `unavailable` | agent CLI not installed |
| 1007 | `budget_exceeded` | task budget reached (on_exceed = pause) |
| 1008 | `external` | git/gh/agent process failed (stderr attached, redacted) |

`error.data` carries `{ "hint_key": "<fluent key>", "details": {…} }` so clients can show a localized fix.

## 4. Method catalog (protocol v1)

Methods are grouped by namespace. Lanes add methods additively after M1; each method is a typed request/response pair in `highset-protocol`. Every list method supports `limit`/`cursor`.

| Namespace | Methods | Owner lane |
|---|---|---|
| `daemon` | `status`, `shutdown`, `restart` | core |
| `events` | `subscribe {topics, filter}` → notifications `event`; `unsubscribe` | core |
| `ui` | `bootstrap` (one call that returns everything the TUI needs for its first frame: workspaces, projects, task summaries, active sessions, attention counts, today's cost) | core/tui |
| `config` | `get {scope}`, `resolve {project?, task?}` (values with origins), `set {scope, key, value}` | core |
| `workspace` | `list`, `create`, `update`, `remove` | core |
| `project` | `list`, `get`, `add {path, workspace?, name?}`, `update`, `remove`, `link_repo` | core |
| `task` | `list {filters}`, `get`, `create`, `update`, `move {to, skip_reason?}`, `remove`, `capture {text, project?}`, `start {agent?, profile?, mode?, prompt?, mentions?}`, `advance`, `done {merge: local\|pr}`, `next_step` | core / platform |
| `gate` | `approve {task, gate, note?}`, `reject {task, gate, reason}` | platform |
| `session` | `list`, `get`, `start`, `stop {grace?}`, `attach {id}` → snapshot + `session.output` notifications, `detach`, `input {bytes\|text}`, `resize {cols, rows}`, `prompt {text, mentions?}` (ACP follow-up), `approve {attention_id, option}`, `deny {attention_id}`, `transcript {id, cursor}` | agents |
| `attention` | `list`, `resolve` | agents |
| `kb` | `list`, `get`, `add`, `update`, `remove`, `ingest {path\|url}`, `move_level` | context |
| `pack` | `list`, `get`, `create`, `update`, `enable {scope}`, `disable {scope}` | context |
| `profile` | `list`, `get`, `resolve {project?, task?}`, `create`, `update` | context |
| `mcp_server` | `list`, `add`, `remove`, `enable {scope}`, `disable {scope}` | context |
| `context` | `compile {target}` → bundle summary, `explain {target}`, `sync {project, dry_run}` → per-file diff | context |
| `mention` | `complete {prefix, kind?}`, `resolve {text}` | context |
| `inbox` | `list`, `get`, `approve {id, edited_body?}`, `reject {id, reason?}` | context |
| `search` | `query {q, filters}`, `fuzzy {q, kinds}` | platform |
| `cost` | `summary {scope, since?}`, `budget_set`, `budget_list` | agents |
| `method` | `standup {since?}`, `weekly`, `adr_new {title}`, `templates` | platform |
| `git` | `diff {task}`, `merge {task}`, `pr_create {task, draft?}` | agents |
| `plugin` | `list`, `inspect {source}` (permissions preview), `add {source, approve_hash}`, `update`, `remove` | platform |
| `secret` | `set {name, value}`, `list`, `remove`. **There is no `get`.** | platform |
| `hook` | `signal {session_token, kind}` (agent-native hook bridge), `list`, `test {event}` | platform/agents |
| `backup` | `now`, `list`, `restore {date, dry_run}` | platform |
| `doctor` | `run` → list of checks with status and hints | platform |
| `mcp` | MCP-shim-only methods mirroring the MCP tools (see `mcp-server.md`) | context |

Notifications from daemon to client: `event` (a subscribed `core::Event`), `session.output` (PTY bytes, base64, sequence-numbered), `daemon.shutting_down`.

## 5. Session output streaming

- `session.attach` returns `{ snapshot: { cols, rows, screen: <vt100 contents formatted as ANSI>, scrollback_lines }, seq }`. Then `session.output {id, seq, data_b64}` notifications follow, coalesced to ≤ 60 per second per client.
- If a client falls behind (more than 4 MiB pending), the daemon drops the queue, sends `session.resync {id}`, and the client calls `attach` again.
- ACP sessions don't use `session.output`. Their structured events arrive through `event` notifications with topic `session.event`.

## 6. Service registry

`highset-daemon` exposes:

```rust
pub trait Service: Send + Sync + 'static {
    fn name(&self) -> &'static str;
    fn register(&self, router: &mut Router);         // add JSON-RPC handlers
    async fn start(&self, ctx: ServiceCtx) -> Result<()>; // spawn actors, subscribe to bus
    async fn stop(&self) -> Result<()>;
}
```

`ServiceCtx` gives access to the store handle, the event bus, config, paths, and handles to the other services the daemon wires together. Each lane adds `services/<area>/mod.rs` and one line in `services/mod.rs`.

## 7. Budgets

- Handshake + one request round trip ≤ 5 ms locally.
- `ui.bootstrap` ≤ 40 ms with 10k tasks (only summary fields; details are fetched lazily).
- Idle: no timers faster than 1 Hz. The scheduler wakes at most once per minute.

## Verified facts

_(Builder: record decisions on lock crate, framing details, measured latencies.)_
