# Spec: search (`highset-search`, daemon `services/search`)

Requirements: SRCH-1…3, PERF-7/8/9. ADRs: 0015.

## 1. Full-text (tantivy)

- **Index location:** `state/search/`. It is a cache, rebuildable with `highset reindex`.
- **Documents indexed:** tasks (title + body), knowledge items (title + body + tags), spec artifacts (`spec.md`, `plan.md`, `tasks.md`), session summaries, session transcripts (message text and ANSI-stripped output, chunked by ~2 KiB with the session id), and ADRs.
- **Schema:**

  | Field | Type | Notes |
  |---|---|---|
  | `doc_id` | STRING, stored | `task:prj…:T-0042`, `kb:kb_…`, `spec:…`, `ses:ses_…:chunk-7` |
  | `doc_type` | STRING, fast | task, kb, spec, summary, transcript, adr |
  | `project`, `workspace` | STRING, fast | |
  | `title` | TEXT, stored | boosted ×3 |
  | `body` | TEXT | |
  | `tags`, `labels`, `status`, `agent`, `profile`, `kind`, `level` | STRING, fast | filterable |
  | `audience_agents`, `audience_profiles` | STRING | for MCP-side filtering |
  | `updated_at` | DATE, fast | sort/filter |

- **Tokenizer:** the default tokenizer with lowercase and ASCII folding, so Spanish accents match unaccented queries. No stemming in v0.1.
- **Updates:** incremental, from bus events. Commits are batched every ≤ 1 s or every 500 docs. A reader reloads after each commit.
- **Query syntax** for users: free text plus `key:value` filters (`status:review`, `label:auth`, `type:kb`, `kind:note`, `agent:codex`, `project:highset`, `since:7d`). Quoted phrases are supported.
- **Corruption** or a schema version change triggers a background rebuild with a warning log. Search keeps answering from the SQLite LIKE fallback meanwhile.

## 2. Fuzzy (nucleo)

- Used by the command palette, the mention picker and `search.fuzzy`.
- **Candidates:** actions, workspaces, projects, tasks (`T-0042 title`), knowledge slugs and titles, sessions, profiles, packs, and files in the current worktree (gitignore-aware walk, cached and refreshed on watcher events).
- **Where it runs:** in the TUI process, over candidate lists cached locally and kept fresh by events. The daemon also exposes it (`search.fuzzy`) for the CLI and MCP.

## 3. Budgets

- PERF-7: ≤ 10 ms per keystroke over 50k fuzzy candidates.
- PERF-8: full-text query p95 ≤ 50 ms over 10k documents.
- PERF-9: full reindex of 10k documents ≤ 10 s.

## Verified facts

_(Builder: tantivy and nucleo versions, measured numbers.)_
