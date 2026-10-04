# Spec: agents and sessions (`highset-agents`, daemon `services/sessions`, `services/attention`)

Requirements: AGT-1…10, UX-5/6, COST-1. ADRs: 0003, 0005, 0006.

> ⚠️ **Verify before implementing.** Agent CLIs change quickly. Every flag, binary name, config path, log path and protocol detail in §5 is a *starting hypothesis* written on 2026-10-03. Each adapter task starts with a short spike: read the agent's current official docs and source, then record the results in "Verified facts" at the end of this file. If reality differs, update this spec (and write an ADR if a decision changes).

## 1. Adapter trait

```rust
#[async_trait::async_trait]
pub trait AgentAdapter: Send + Sync {
    fn kind(&self) -> AgentKind;                         // ClaudeCode | Codex | Opencode | Cursor | Custom(String) | Fake
    fn display_name(&self) -> &'static str;
    async fn detect(&self) -> Detection;                 // installed?, binary path, version, ACP available?, notes
    fn modes(&self, det: &Detection) -> Vec<SessionMode>; // ordered by preference
    async fn launch(&self, spec: LaunchSpec) -> Result<RunningSession, AdapterError>;
    fn harvester(&self) -> Option<Arc<dyn TranscriptHarvester>>; // reads the agent's own logs for usage/transcript
    fn launch_injection(&self, bundle: &ContextBundle, ctx: &InjectionCtx) -> LaunchInjection; // per-session flags/env
}

pub struct LaunchSpec {
    pub session_id: SessionId,
    pub session_token: SecretString,     // for the MCP shim
    pub task: Option<TaskRef>,
    pub cwd: PathBuf,                    // the task worktree (or project root for ad-hoc sessions)
    pub mode: SessionMode,
    pub bundle: Arc<ContextBundle>,      // compiled context; files already emitted into cwd by the daemon
    pub initial_prompt: Option<String>,  // already expanded (attachments + mentions)
    pub env: Vec<(String, SecretOrPlain)>, // includes HIGHSET_SESSION_ID, HIGHSET_SESSION_TOKEN, HIGHSET_SOCKET, HIGHSET_TASK
    pub pty_size: Option<(u16, u16)>,
}

pub struct RunningSession {
    pub events: mpsc::Receiver<SessionEvent>, // normalized
    pub control: Box<dyn SessionControl>,     // input, resize, prompt, respond_permission, cancel, kill
    pub pid: Option<u32>,
}
```

The adapter registry is built at daemon start. Detection results are cached for 10 minutes and refreshed on `agent list` or `doctor`.

## 2. Normalized `SessionEvent`

```
Started { mode, agent_version, protocol }
Output { bytes }                              // PTY only; not persisted raw by default
AgentMessage { text, final: bool }            // ACP message chunks aggregated
Thought { text }                              // if the agent exposes it; stored but hidden by default
ToolCall { id, title, kind, input_summary, status }
ToolCallUpdate { id, status, output_summary }
Plan { entries: [{ content, status }] }
FileDiff { path, patch }
PermissionRequest { request_id, title, detail, options: [{ id, label, kind: allow_once|allow_always|reject }] }
InputRequested { reason }                     // PTY heuristic or native hook
Usage { input, output, cache_read, cache_write, model, cost_usd? }
Progress { message, percent? }                // via MCP highset_progress_report
Error { message }
Ended { reason: completed|canceled|error|killed, exit_code? }
```

The supervisor persists every event except `Output` to `transcript.jsonl`. For PTY sessions, it also stores ANSI-stripped output chunks. All persisted text is redacted first.

## 3. Modes

### 3.1 ACP (preferred)

- HighSet is the **ACP client**. The agent runs as a subprocess speaking ACP (JSON-RPC) over stdio. Use the official `agent-client-protocol` crate, pinned; record the version in `session.toml`.
- **Client capabilities** advertised:
  - File system read/write, limited to the session `cwd`. Requests outside it are refused with an error.
  - Terminal capability, if the crate version supports it and the agent asks for it. It runs commands in the worktree, through the same permission rules.
- `session/new` gets `cwd` and the MCP servers from the bundle; the HighSet MCP server is always included. Then comes `session/prompt` with `initial_prompt`.
- `session/update` notifications map to `AgentMessage`, `ToolCall*`, `Plan` and `FileDiff`.
- `session/request_permission` becomes a `PermissionRequest`, then an attention item. The owner's answer is relayed back. A timeout produces no automatic decision; the session simply waits.
- `cancel` sends the ACP cancel notification; `kill` terminates the process after a 3 s grace period.

### 3.2 PTY (fallback for every agent)

- `portable-pty`, spawned in `cwd` with `TERM=xterm-256color` and the env from `LaunchSpec`.
- A ring buffer per session (default 2 MiB) holds raw bytes; a `vt100::Parser` maintains the screen for snapshots (default 5,000 scrollback lines).
- Input: bytes from attached clients. Resize: `session.resize` → PTY resize (the child gets SIGWINCH).
- **"Needs input" detection,** in order of preference:
  1. Native agent hooks call `highset hook signal --kind needs_input|finished|permission` (see §5).
  2. Heuristic: no output for `idle_secs` (default 4 s) **and** the cursor is on a line that matches the adapter's prompt regex. This raises an `InputRequested` attention item at most once per idle period.

### 3.3 Headless

- Used for automation and for the `agent:<kind>` LLM backend (`llm.md`).
- Run the agent's non-interactive JSON-stream mode and parse each line into `SessionEvent`s. No input is possible, except cancellation.

## 4. Session supervisor (daemon)

- `session.start`:
  1. Validate the task, then ensure its worktree via the git service.
  2. Compile the bundle (context service), emit sync targets into the worktree, and build the `LaunchSpec`.
  3. Pick the mode: requested, or else the first supported one from `modes()`.
  4. Launch, persist `session.toml`, and publish `session.started`.
- Concurrency: by default at most one running session per task (config `sessions.max_per_task = 1`). No global limit, but a warning above `sessions.warn_above = 6`.
- Sessions outlive client connections. On daemon shutdown, the default is to stop them gracefully. On daemon start, sessions recorded as running are marked `lost` (NFR-REL-2).
- At the end of a session:
  1. Write `diff.patch` (git service).
  2. Finalize the usage aggregates.
  3. Publish `session.ended`.
  4. Enqueue the internal LLM tasks: summary and memory extraction.
  5. Raise a `session_finished` attention item.

## 5. Per-agent adapter notes (hypotheses to verify)

| | Claude Code | Codex CLI | opencode | Cursor CLI |
|---|---|---|---|---|
| Binary | `claude` | `codex` | `opencode` | `cursor-agent` |
| ACP | Via the Claude Code ACP adapter maintained by Zed (npm package, e.g. `@zed-industries/claude-code-acp`) | Via the `codex-acp` adapter (Zed) | Native subcommand (`opencode acp`) | Unknown; check. PTY + headless otherwise |
| PTY | `claude` interactive | `codex` interactive | `opencode` TUI | `cursor-agent` interactive |
| Headless | `claude -p --output-format stream-json` | `codex exec --json` | check | `cursor-agent -p --output-format stream-json` |
| Instruction file | `CLAUDE.md` (can import `@AGENTS.md`) | `AGENTS.md` | `AGENTS.md` | `AGENTS.md`, `.cursor/rules/*.mdc` |
| Config / MCP | `.claude/settings.json`, `.mcp.json`; launch flags such as `--mcp-config`, `--append-system-prompt`, `--settings` | `~/.codex/config.toml` (MCP servers, model, approval/sandbox); isolate via `CODEX_HOME`? Decide in the A-006 spike (ADR) | `opencode.json` | `.cursor/mcp.json` |
| Native hooks for attention | `Notification` and `Stop` hooks in settings call `highset hook signal` | `notify` config runs a program on turn end | check | check |
| Transcript / usage logs | JSONL under `~/.claude/projects/<encoded-path>/` | Rollout JSONL under `~/.codex/sessions/` | check | check |

Adapters must work even when the agent's ACP adapter is not installed: `detect()` reports `acp: false` with an install hint, and `modes()` falls back to PTY.

The `Fake` adapter (`tools/fake-agent`) implements all three modes from scripted scenarios. It is used in every integration test.

## 6. Transcript harvesters

When a mode doesn't expose usage or the full transcript (typically PTY), the harvester tails the agent's own session log for the session's `cwd` and start time.

- It extracts `Usage` records and, optionally, message text for search.
- Harvesting is best-effort and read-only. It never modifies agent files.
- Parsers are tested against redacted fixture files committed in `crates/highset-agents/tests/fixtures/<agent>/`.

## 7. Attention and notifications (`services/attention`)

- Sources: permission requests, input needed, session finished, gate pending, budget alerts, pending proposals, hook failures.
- OS notifications via `notify-rust`, which covers macOS and Linux (D-Bus). They are rate-limited to one per session per 10 s and can be toggled per kind (`[notifications]`). The title names the project and task, e.g. "highset · T-0042 needs approval".
- `attention.list` is ordered by urgency: permission > input > gate > budget > finished > proposal.
- The TUI status bar shows the count of unresolved items by kind.

## 8. Autonomy (AGT-7)

Default `semi`:

- Agents run with permissive rules **inside** the worktree (read/edit files, run the project's build/test commands).
- Writes to `.highset/**`, `git push`, network-heavy commands and anything outside `cwd` are denied or asked, as compiled from the profile's `PermissionSet` (`harness.md`).
- Flow gates are human-only (`core-domain.md` §3).

`supervised` asks for everything that isn't a read; `autonomous` allows everything inside `cwd` except the hard denies. The level is set per profile, project or task.

## Verified facts

_(Builder: one subsection per agent. Example:)_

### Claude Code
- Verified on: _date_ · Version: _x.y.z_ · Sources: _URLs_
- ACP adapter: _package, version, how to launch_
- Headless flags: _…_
- Config files and precedence: _…_
- Hooks used for attention: _…_
- Log location and format: _…_
