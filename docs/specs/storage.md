# Spec: storage (`highset-store`)

Requirements: ORG-3, CTX-1/2, DATA-1/4/5, NFR-REL-1. ADRs: 0007, 0008, 0011, 0022.

## 1. Principles

1. **Files are the truth.** Every durable fact lives in a Markdown or TOML file (or JSONL for append-only logs) under the content dir or a project's `.highset/`. SQLite and tantivy are caches, and `highset reindex` rebuilds them from scratch.
2. **One writer.** Only the daemon writes content. The CLI and TUI go through the daemon.
3. **Atomic writes:** write `<file>.tmp-<rand>`, fsync it, rename over the target, fsync the directory.
4. **Round-trip safety.** Rewriting a file after changing one field preserves everything else byte-for-byte: other frontmatter keys and their order, comments in TOML (use `toml_edit`), and the Markdown body.
5. **Human-editable.** Users may edit any file in `$EDITOR`. The watcher re-validates; invalid files raise an attention item (`kind: hook_failed`, title "Invalid file") and are skipped, never deleted.

Layout: see `ARCHITECTURE.md` §4. That layout is part of the contract.

## 2. File formats

Markdown files use **YAML frontmatter** (`---` fences), the de-facto standard used by Claude Code agents, Agent Skills and most static-site tooling (ADR-0011). Standalone config files are **TOML**.

### 2.1 `config.toml` (global), `workspace.toml`, `project.toml`

```toml
# ~/.highset/config.toml
[ui]
language = "auto"          # auto | en | es
mouse = true
theme = "terminal"         # terminal | <preset name>

[paths]
content_dir = "~/.highset/content"   # may point into iCloud/Syncthing/git
state_dir = "~/.highset/state"       # never synced

[git]
worktrees_dir = "~/.highset/worktrees"
branch_prefix = "hs/"

[sync]
targets = ["claude-code", "agents-md", "codex"]   # also: opencode, cursor, gemini

[notifications]
on_finished = true
on_attention = true

[llm.tasks]
summarize_session = "anthropic:claude-haiku-4-5"
extract_memory   = "anthropic:claude-haiku-4-5"
tag_knowledge    = "anthropic:claude-haiku-4-5"
idea_to_spec     = "anthropic:claude-opus-5-5"
fallback = "agent:claude"   # use the installed Claude Code headless when no API key
```

```toml
# <repo>/.highset/project.toml
id = "prj_01JB…"
name = "HighSet"
slug = "highset"
workspace = "work"
linked_repos = []
default_branch = "main"

[flow]
default = "feature"
strictness = "guide"       # guide | strict | info

[verify]
test = "cargo test --workspace"
lint = "cargo clippy --workspace --all-targets -- -D warnings"
format = "cargo fmt --all -- --check"

[specs]
dir = "specs"              # Spec Kit-compatible location

[sync]
targets = ["claude-code", "agents-md", "codex"]
```

### 2.2 Task file `.highset/tasks/T-0042-add-login.md`

```markdown
---
id: tsk_01JB…
number: T-0042
title: Add login with GitHub
flow: feature
status: plan
priority: high
labels: [auth]
epic: accounts
profile: builder
agent: claude-code
depends_on: [T-0040]
packs: [rust-backend]
links:
  - { kind: issue, target: "https://github.com/acme/app/issues/12" }
branch: hs/T-0042-add-login
gates:
  approve_spec: { state: approved, by: owner, at: 2026-10-20T10:12:00Z }
created_at: 2026-10-19T09:00:00Z
updated_at: 2026-10-20T10:12:00Z
---

Users must sign in with GitHub OAuth…

## Acceptance criteria
- [ ] …

## History
- 2026-10-20 10:12 spec → plan (gate approve_spec approved)
```

`cost` and `worktree` are cache-only (SQLite), not written to the file. That keeps diffs quiet.

### 2.3 Knowledge item

```markdown
---
id: kb_01JB…
title: Error-handling conventions
kind: doc
tags: [rust, conventions]
audience: { agents: [claude-code], profiles: [], tasks: [] }
covers: ["crates/*/src/error.rs"]
source: { kind: url, url: "https://…", fetched_at: 2026-10-20T08:00:00Z }
created_at: …
updated_at: …
---
Body…
```

Assets (images, PDFs, data files) sit next to the item in an `assets/` folder at the same level. The item's `asset:` key holds the relative path.

### 2.4 Session directory `content/sessions/<project-id>/<session-id>/`

| File | Format | Notes |
|---|---|---|
| `session.toml` | TOML | Entity fields (core-domain §2), written at start and updated at end |
| `transcript.jsonl` | JSON Lines | One normalized `SessionEvent` per line: `{"t":"…","seq":n,"kind":"agent_message","data":{…}}`. PTY output is stored as `{"kind":"output_text","data":{"text": <ANSI-stripped>}}` chunks of ≤ 8 KiB |
| `pty.raw.zst` | zstd | Optional raw PTY bytes for faithful replay; off by default (`[sessions] keep_raw_pty = false`) |
| `diff.patch` | unified diff | Worktree vs. base at session end |
| `summary.md` | Markdown | Written by the `summarize_session` task |
| `usage.jsonl` | JSON Lines | `UsageRecord` per line |

Everything here is redacted before writing (`security.md`).

### 2.5 Other files

- `content/tasks/I-0001-<slug>.md`: **global inbox** items captured outside any project (`highset add` with no project). Same format as task files; numbered `I-NNNN`. `task move-to` moves one into a project and assigns it a project `T-NNNN` number.
- `content/inbox/<prp_id>.md`: a Proposal, with frontmatter (kind, target, provenance, state) and the proposed content as the body.
- `content/journal/YYYY/MM/DD.md`: append-only daily journal with `## HH:MM project · T-id` entries.
- `content/mcp/servers.toml`: see `harness.md`.
- `content/plugins.lock`: see `plugins.md`.
- `content/pricing.toml`: see `costs.md`.

## 3. SQLite cache (`state/highset.db`)

WAL mode and `synchronous=NORMAL`. A single connection is owned by the store actor (a dedicated OS thread); reads and writes are serialized through channels. Prepared statements are cached. Migrations are embedded (`rusqlite_migration`); the schema version is in `user_version`.

Tables (sketch; the builder finalizes them in P1-003 and documents them here):

```sql
CREATE TABLE file_index (path TEXT PRIMARY KEY, kind TEXT NOT NULL, mtime_ns INTEGER, size INTEGER, hash BLOB);
CREATE TABLE workspaces (id TEXT PRIMARY KEY, slug TEXT UNIQUE, name TEXT, path TEXT);
CREATE TABLE projects (id TEXT PRIMARY KEY, workspace_id TEXT, slug TEXT, name TEXT, root TEXT UNIQUE, default_branch TEXT);
CREATE TABLE tasks (id TEXT PRIMARY KEY, project_id TEXT, number INTEGER, title TEXT, flow TEXT, status TEXT,
  blocked TEXT, priority TEXT, epic TEXT, profile TEXT, agent TEXT, branch TEXT, worktree TEXT,
  created_at TEXT, updated_at TEXT, path TEXT, UNIQUE(project_id, number));
CREATE TABLE task_labels (task_id TEXT, label TEXT);
CREATE TABLE task_deps (task_id TEXT, depends_on_number INTEGER);
CREATE TABLE task_links (task_id TEXT, kind TEXT, target TEXT, title TEXT);
CREATE TABLE kb_items (id TEXT PRIMARY KEY, slug TEXT, title TEXT, kind TEXT, level TEXT, level_ref TEXT,
  path TEXT UNIQUE, updated_at TEXT);
CREATE TABLE kb_tags (kb_id TEXT, tag TEXT);
CREATE TABLE kb_audience (kb_id TEXT, axis TEXT, value TEXT);
CREATE TABLE sessions (id TEXT PRIMARY KEY, project_id TEXT, task_id TEXT, agent TEXT, mode TEXT, profile TEXT,
  state TEXT, started_at TEXT, ended_at TEXT, dir TEXT);
CREATE TABLE usage (session_id TEXT, at TEXT, model TEXT, input INTEGER, output INTEGER, cache_read INTEGER,
  cache_write INTEGER, cost_usd REAL);
CREATE TABLE attention (id TEXT PRIMARY KEY, kind TEXT, session_id TEXT, task_id TEXT, created_at TEXT,
  resolved_at TEXT, payload TEXT);
CREATE TABLE proposals (id TEXT PRIMARY KEY, kind TEXT, state TEXT, path TEXT, created_at TEXT);
CREATE TABLE budgets (scope TEXT, scope_id TEXT, period TEXT, limit_usd REAL, limit_tokens INTEGER, on_exceed TEXT);
```

Rules:

- No table holds data that is not derivable from files, **except** the `attention` table, which is runtime state and is rebuilt from session records after a restart.
- Budgets are stored in config files and mirrored into the cache.
- `highset reindex` drops and rebuilds everything from content. A test asserts that the rebuilt cache equals the incremental one.

## 4. File watcher

- `notify` with debouncing (200 ms) over: the content dir, every registered project's `.highset/`, `specs/`, and `docs/decisions/`.
- On change: re-read the file, validate it, update the cache, publish an event, and update the search index.
- Ignore temp files written by the daemon itself (by suffix and a short in-memory suppression set).
- Budget: an external edit is reflected in the cache and pushed to clients in ≤ 1 s.

## 5. Backups (D-011)

- Daily snapshot (first daemon start after 03:00 local, or `highset backup now`) of the content dir into `~/.highset/backups/YYYY-MM-DD.tar.zst`.
- Retention: 7 daily + 4 weekly (configurable).
- `highset backup list | restore <date> [--dry-run]`. A restore writes into a fresh dir and swaps it atomically; the previous content is kept as `content.pre-restore-<ts>`.

## 6. Sync readiness (ADR-0022)

- Users may point `content_dir` at a synced folder. HighSet never puts SQLite, sockets or locks there.
- Session and journal files are per-machine-safe: their filenames embed IDs or dates, so two machines don't write the same file.
- Sync providers (v0.2) plug in behind a `SyncProvider` trait. Not built in v0.1.

## Verified facts

_(Builder: record crate versions chosen and any behavior verified, with dates.)_
