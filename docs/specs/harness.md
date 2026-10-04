# Spec: harness: profiles, permissions, hooks, MCP registry (`highset-context`, daemon `services/hooks`)

Requirements: AGT-7, AGT-10, HRN-1…8. ADRs: 0009, 0011, 0016.

## 1. Profiles

A profile is an `*.agent.md` file: YAML frontmatter for the configuration and a Markdown body for the instructions. It is compatible in spirit with Claude Code subagent files and VS Code/Copilot `.agent.md` files, so the same file can be emitted as a subagent.

```markdown
---
name: builder
description: Implements the approved plan with tests, inside the task worktree.
extends: null                 # or the name of another profile to inherit from
agents: []                    # agents this profile supports; empty = all
autonomy: semi                # supervised | semi | autonomous
model:                        # agent-native model identifiers; omit a key = agent default
  claude-code: opus
  codex: null
packs: [software-core]
mcp_servers: [github]         # names from the MCP registry; "highset" is always added
skills: []                    # skill names (from any level or pack)
commands: [review-diff]       # *.prompt.md names
subagents: [reviewer]         # other profiles emitted as subagents where supported
permissions:
  allow: ["read:**", "edit:**", "bash:<verify>", "bash:git status", "bash:git diff*", "bash:git add *", "bash:git commit *"]
  deny:  ["edit:.highset/**", "bash:git push*", "read:.env*", "bash:rm -rf *"]
agent_hooks: {}               # raw per-agent hook passthrough (advanced)
env:
  - { name: GITHUB_TOKEN, value: "secret:github_token" }
---
You implement the task's approved plan. Work only inside the current worktree…
```

### Built-in profiles (AGT-10)

They ship inside the binary as embedded `*.agent.md` files. Override one by creating a file with the same name at any level, or extend it with `extends:`.

| Profile | Purpose | Autonomy | Edits allowed | Key instructions |
|---|---|---|---|---|
| `researcher` | Investigate, read docs and the web, summarize | semi | `specs/**/research.md` only | Cite sources; propose knowledge items with `highset_knowledge_propose`; never change code |
| `planner` | Write spec, plan and tasks from templates | semi | `specs/**`, `docs/**` | Follow the spec template; make acceptance criteria testable; list open questions explicitly |
| `builder` | Implement the approved plan with tests | semi | everything except `.highset/**` | Tests first where practical; run the verify commands before requesting review; small commits |
| `reviewer` | Review the diff against spec, acceptance criteria and conventions | semi | none (read-only); may run verify commands | Output a verdict (PASS / CHANGES) with findings by severity; never fix the code itself |
| `debugger` | Reproduce, isolate and fix a bug | semi | everything except `.highset/**` | Write a failing test first; explain the root cause in the session summary |
| `documenter` | Keep docs, changelog and ADRs current | semi | `**/*.md`, `docs/**`, `CHANGELOG.md` | Match existing tone; update docs flagged as stale |

Each body is a well-written prompt of 150–400 words, drafted in task B-005 and reviewed by the reviewer subagent. The bodies are English and agent-neutral.

## 2. Layering (HRN-4)

- Profiles resolve by name. **Lookup order (most specific wins):** task overrides → local → project → workspace → global → plugin → built-in.
- A more specific file with the same name **replaces** the less specific one, unless it sets `extends: <same-name>`. In that case it merges: maps merge deeply, lists are replaced unless the key is prefixed with `+`, and the body is appended under a `## Project-specific` heading.
- Task-level tweaks without a new file go in the task frontmatter:

  ```yaml
  profile: builder
  profile_overrides:
    model: { claude-code: sonnet }
    +permissions: { deny: ["edit:migrations/**"] }
  ```

- `highset profile show builder --resolved --task T-0042` prints the result with the origin of each field.

## 3. Autonomy levels (AGT-7)

| Level | Inside the worktree | Outside the worktree / risky |
|---|---|---|
| `supervised` | Reads allowed; edits and commands ask | Denied or ask |
| `semi` (default) | Edits and the project's verify/build commands allowed; other commands ask | `git push`, network installs, `.highset/**` edits, anything outside cwd: denied or ask |
| `autonomous` | Everything allowed except hard denies | Hard denies stay: `.highset/**` edits, force pushes, reading `.env*` |

Flow gates are always human (`core-domain.md` §3), whatever the autonomy level.

## 4. Neutral permission syntax

```
read:<glob>        edit:<glob>        bash:<command pattern>
mcp:<server>/<tool-or-*>              net:<host-or-*>
```

- Globs are relative to the session cwd. Command patterns match the command line with `*` wildcards.
- `<verify>` expands to the project's configured verify commands (`project.toml [verify]`).
- **Evaluation:** deny beats allow; anything not matched falls back to the autonomy level's default (ask or allow).

## 5. Compilation per agent

Each sync target translates the `PermissionSet`. A target that cannot express a rule exactly approximates it **in the safer direction** and reports a warning in `context explain` (e.g. "codex: per-command rules not supported, approximated with approval policy on-request").

| Agent | How permissions are expressed (verify in the target tasks) |
|---|---|
| Claude Code | `permissions.allow` / `permissions.deny` in `.claude/settings.json` with tool patterns (e.g. `Read(...)`, `Edit(...)`, `Bash(cargo test:*)`, `mcp__server__tool`) |
| Codex | Coarse: sandbox mode (read-only / workspace-write / full access) and approval policy, mapped from autonomy and the deny list |
| opencode | Its permission config (allow / ask / deny per tool, bash patterns) |
| Cursor CLI | Its CLI permission config (allow/deny lists), if available |
| Any agent in ACP mode | **HighSet enforces too:** permission requests that match an allow rule are auto-approved, deny rules are auto-rejected, and the rest go to the owner. fs/terminal client requests outside cwd are always refused. |

## 6. Hooks

There are two kinds.

**Agent-native hooks** run inside the agent; they are listed in the bundle's `hooks`. HighSet installs at least the **attention bridge**, so each agent reports "needs input", "finished" and "permission" precisely:

```
highset hook signal --kind needs_input|finished|permission
```

It uses `HIGHSET_SESSION_TOKEN` from the env. Profiles can also pass raw agent hooks with `agent_hooks.<agent>`.

**HighSet hooks** are run by the daemon (D-010). They are defined in `hooks.toml` at the global, project or plugin level, or in a profile's `hooks:` list:

```toml
[[hook]]
event = "pre_merge"              # session.start | session.end | task.status_changed | pre_commit | pre_merge | attention
run = ["./scripts/check-licenses.sh"]   # argv form; no shell unless run = ["sh", "-c", "..."] is written explicitly by the user
when = { profile = "builder" }   # optional filters: project, profile, agent, status_to
timeout_secs = 60
```

- **Environment:** `HIGHSET_EVENT`, `HIGHSET_PROJECT_ROOT`, `HIGHSET_TASK`, `HIGHSET_SESSION_ID`, `HIGHSET_WORKTREE`, `HIGHSET_STATUS_FROM/TO`. The full event JSON goes to stdin.
- **Blocking:** for `pre_commit` and `pre_merge`, a non-zero exit blocks the action and shows the hook's stderr (redacted). For the other events, failures raise a `hook_failed` attention item and never block.
- Hooks from plugins only run if the plugin's approved permissions include `hooks` and the commands they call.

## 7. MCP server registry (HRN-8)

```toml
# content/mcp/servers.toml (global); projects may add .highset/mcp.toml
[servers.github]
description = "GitHub API (issues, PRs)"
transport = "stdio"                   # stdio | http
command = "github-mcp-server"
args = ["stdio"]
env = { GITHUB_TOKEN = "secret:github_token" }

[servers.docs-search]
transport = "http"
url = "https://example.com/mcp"
headers = { Authorization = "secret:docs_token" }   # expanded as "Bearer <value>" if prefixed in config
```

- **Enable** per profile (`mcp_servers:`), per project (`[mcp] enabled = [...]`), per pack, or per task.
- **HighSet's own server is always included:** `highset = { command = "highset", args = ["mcp", "serve"] }`.
- **Secrets** (`secret:<name>`) resolve only at launch, into the agent process env. In written config files, servers get env var references (`${GITHUB_TOKEN}` or the agent's equivalent syntax), never values.
- `highset mcp-server add github -- github-mcp-server stdio` registers a server. `doctor` checks that registered commands exist.

## Verified facts

_(Builder: per-agent permission syntax and hook mechanisms confirmed, with dates and sources.)_
