# Spec: knowledge (`highset-context` knowledge module, daemon `services/knowledge`, `services/inbox`)

Requirements: CTX-1…6, CTX-11, CTX-12. ADRs: 0007, 0011, 0020.

## 1. Knowledge items

A knowledge item is a Markdown file with YAML frontmatter (`storage.md` §2.3), plus an optional asset. Kinds: `doc`, `note`, `prompt`, `spec`, `snippet`, `reference`, `image`, `data`, `memory`, `instructions`, `skill`, `profile`, `project_overview`.

## 2. Levels (where an item lives)

| Level | Location | Committed to git? | Visible to |
|---|---|---|---|
| global | `content/knowledge/`, `content/context/` | n/a (personal) | every project |
| workspace | `content/workspaces/<ws>/{knowledge,context}/` | n/a | projects in that workspace |
| project | `<repo>/.highset/{knowledge,context}/` | **yes** | anyone working on the repo |
| local | `<repo>/.highset/local/{knowledge,context}/` | **no** (gitignored) | only this machine, this project |

**Shadowing:** an item with the same slug at a more specific level replaces the less specific one. The order is local > project > workspace > global. `context explain` shows shadowed items.

## 3. Audience (who can see an item), answering the Q5.3 note

The owner asked for knowledge that "only one AI should have, per project". The audience frontmatter restricts visibility on three axes:

```yaml
audience:
  agents: [claude-code]      # only Claude Code sessions see it
  profiles: [reviewer]       # only sessions using the reviewer profile
  tasks: [T-0042]            # only sessions on this task
```

- Empty or missing axes mean "no restriction". Non-empty axes are ANDed.
- Combined with levels: "only Claude, only in this project, not shared with the team" is `level: local` plus `audience.agents: [claude-code]`.
- Audience applies **everywhere** context flows: compiled files, attachments, the on-demand index, MCP search/get results, and mention resolution.
- **Committed generated files** (e.g. `AGENTS.md` in the repo) are shared by every agent and person. So they only include items with **no audience restriction** that are not `local`. Restricted items reach their audience through launch-time injection (per-session flags/files excluded from git) and MCP. This is enforced and tested (B-003).

## 4. Convention files (Q5.4 note)

The owner wants Markdown files "named for a purpose", like `CLAUDE.md` or `AGENTS.md`. HighSet recognizes these names in any `context/` directory, at any level:

| File | Kind | Meaning | How it is used |
|---|---|---|---|
| `PROJECT.md` | `project_overview` | What this project is, key commands, layout | Always inline, first |
| `*.instructions.md` | `instructions` | Rules and conventions. Optional `applyTo: ["src/**/*.rs"]` frontmatter, the same convention as VS Code/GitHub Copilot instruction files | Always inline (or scoped by `applyTo` where the target supports scoped rules) |
| `*.prompt.md` | `prompt` | A reusable prompt / slash command (`name`, `description`, `arguments` in frontmatter) | Emitted as agent commands; available as `@pack`/palette actions |
| `*.agent.md` | `profile` | A harness profile or subagent (`harness.md`) | Profiles and subagents |
| `skills/<name>/SKILL.md` | `skill` | An Agent Skill (open standard: `name` + `description` frontmatter, body, optional resources) | Emitted to agents that support skills; listed on demand for the others |
| `MEMORY.md` | `memory` | Approved memories (append-only sections with provenance) | Always inline (budgeted) |
| anything else `*.md` | from frontmatter `kind`, default `doc` | Normal knowledge | On demand (MCP), via packs, or mentions |

`highset context ls` lists the recognized files per level, with their kinds.

## 5. Ingestion (CTX-6)

`highset kb add <source>` or `kb.ingest`:

| Source | Processing | Result |
|---|---|---|
| Markdown / text file | Copy; add frontmatter if missing | Item of kind `doc` (or `--kind`) |
| Code file | Wrap in a fenced block with language and path | `snippet` |
| URL | Fetch (reqwest + rustls, 15 s timeout, 10 MiB cap), extract the main content (readability-style; crate chosen in B-002), convert to Markdown, keep the title and URL | `reference` with `source.url`, `fetched_at` |
| PDF | Extract text (pure-Rust crate first; fall back to `pdftotext` if installed); keep the PDF as an asset | `reference` + asset |
| Image | Copy as an asset; sidecar Markdown with title, alt and an optional description (an LLM description is a v0.2 option) | `image` |
| CSV / JSON | Copy as an asset; the sidecar has a schema preview (columns and types, row count, first 5 rows) | `data` |

- Dedupe by content hash: re-adding the same content updates the existing item instead of duplicating it.
- Files over 20 MiB need `--force`.
- If configured (`[llm] auto_tag = true`), the `tag_knowledge` internal task suggests tags.
- No network access except for URL ingestion.

**v0.2:** repo → knowledge graph via graphify (run graphify if installed and import its output as items), plus Notion and Obsidian importers as plugins.

## 6. Memory and proposals (CTX-11)

Proposals come from three places:

1. MCP tools: `highset_memory_propose`, `highset_knowledge_propose`, `highset_decision_propose`.
2. The end-of-session `extract_memory` task (`llm.md`).
3. Staleness detection (§7).

How they are handled:

- Each proposal is a file in `content/inbox/` (`storage.md`). It is **never** visible to agents until approved.
- On approval:
  - `memory` → appended to the target `MEMORY.md` (project by default; the owner can pick a level) as a section with date, source session and task.
  - `knowledge` → a new item at the chosen level.
  - `decision` → a new ADR (`methodology.md`).
  - `doc_update` → a patch applied to the target file.
- Approve, edit-then-approve and reject are all logged in the journal.

## 7. Staleness (CTX-12)

When a task reaches `done`:

1. Take the list of files changed in the task's diff.
2. Find items whose `covers` globs match a changed file. Items without `covers` are a candidate if their body mentions a changed path or a top-level symbol name from the diff (simple text search).
3. For each candidate, create a `doc_update` proposal: "Doc X may be outdated after T-0042", with the relevant diff excerpt. If an LLM is configured, also suggest a patch.

The goal is high precision. When in doubt, don't propose.

## Verified facts

_(Builder: crates chosen for readability extraction, HTML→Markdown and PDF text, with dates and quality notes.)_
