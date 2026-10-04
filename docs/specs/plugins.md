# Spec: plugins (`highset-plugins`, daemon `services/plugins`)

Requirements: PLG-1…7, HRN-8. ADRs: 0012.

## 1. Package layout (declarative, v0.1)

```
acme-rust-pack/
├── highset-plugin.toml
├── context/*.instructions.md  context/*.prompt.md  context/*.agent.md
├── skills/<name>/SKILL.md
├── packs/<name>/pack.toml
├── knowledge/*.md
├── mcp/servers.toml
├── hooks/hooks.toml
├── commands/*.toml            shell commands exposed as palette actions / `highset run <plugin>:<cmd>`
├── flows/*.toml  templates/*
└── locales/{en,es}/*.ftl      strings for the plugin's commands
```

## 2. Manifest

```toml
[plugin]
id = "acme.rust-pack"          # publisher.name, lowercase
name = "Rust pack"
version = "0.3.1"              # SemVer
description = "Rust conventions, cargo skills and a GitHub MCP server"
authors = ["Acme <dev@acme.dev>"]
license = "MIT"
homepage = "https://github.com/acme/highset-rust-pack"
min_highset = "0.1.0"

[permissions]                  # everything the plugin can cause to run or reach
commands = ["cargo", "gh"]     # executables used by hooks/commands/MCP servers
network = ["api.github.com"]   # hosts contacted by its MCP servers/commands (declared; informational in v0.1)
filesystem = ["project:read", "project:write"]   # what its hooks/commands touch
secrets = ["github_token"]     # secret names it requests via secret:<name>
hooks = ["session.end", "pre_merge"]
mcp_servers = ["github"]
```

v0.1 cannot sandbox plugin commands. Permissions are a **consent and review** mechanism:

- the install screen shows them;
- the lockfile pins them;
- an update that changes them requires re-approval;
- HighSet refuses to run a hook or command that uses an executable or secret not listed.

## 3. Sources and lockfile (PLG-2)

| Source syntax | Meaning |
|---|---|
| `github:owner/repo[@ref][#subdir]` | Clone over HTTPS |
| `git:https://…[@ref]` | Any git URL |
| `path:/abs/or/relative` | Local folder (dev mode, watched for changes) |
| `cc:<marketplace>/<plugin>` | A Claude Code marketplace entry (§5) |

- Install location: `content/plugins/<id>/` (a clean checkout at the pinned commit).
- **`content/plugins.lock`:**

  ```toml
  [[plugin]]
  id = "acme.rust-pack"
  source = "github:acme/highset-rust-pack"
  ref = "v0.3.1"
  commit = "9f2c…"
  permissions_hash = "sha256:…"
  approved_at = "2026-10-21T09:00:00Z"
  enabled = true
  ```

- Commands: `plugin inspect <source>` (fetch to temp, show manifest + permissions + contributions, no install), `plugin add`, `plugin update [id]` (fetch, diff permissions, re-approve if changed), `plugin rm` (removes the checkout and every contribution; a test asserts no residue).

## 4. Contribution model

On load, a plugin's files are registered as an extra **level** named `plugin:<id>`, with precedence just above built-ins (`harness.md` §2). Contributions then behave exactly like user files. Contributions are namespaced by plugin id in the UI, and name collisions with user files resolve in favor of the user.

## 5. Claude Code compatibility (PLG-4, PLG-5)

**Import Claude Code plugins.** Read the package's `.claude-plugin/plugin.json` and its folders (commands, agents, skills, hooks, MCP server config) and map them:

| Claude Code | HighSet |
|---|---|
| commands (`commands/*.md`) | `*.prompt.md` contributions |
| agents (`agents/*.md`) | `*.agent.md` contributions (subagents/profiles) |
| skills (`skills/*/SKILL.md`) | skills |
| hooks | **Not auto-enabled.** Listed for review; the user can enable them as Claude Code agent hooks for Claude sessions |
| MCP servers | Registry entries (permissions inferred, marked "review") |

- **Marketplaces:** a marketplace (`.claude-plugin/marketplace.json`) can be added as a source of `cc:` plugins.
- **Standalone Agent Skills:** `highset plugin add path:./my-skill` on a folder containing `SKILL.md` installs it as a skill-only plugin.
- **Distribution:** skills reach Claude Code natively, through `.claude/skills/`. Agents without native skill support see them as on-demand knowledge (name + description in the index, body via MCP).
- **Verify the current formats** of plugin.json, marketplace.json, hooks and skills against Claude Code docs in D-009, and record them under Verified facts.

## 6. Code plugins (v0.2, design notes)

- **Process model:** an out-of-process program speaking JSON-RPC 2.0 over stdio, the LSP/MCP pattern. It is declared in the manifest as `[runtime] command = [...]`, with `capabilities = ["agent_adapter", "importer", "integration", "command"]`.
- **Lifecycle:** HighSet starts it lazily and restarts it on crash with backoff.
- **Permissions:** the same model as §2, plus an optional OS sandbox (macOS `sandbox-exec` profile / Linux Landlock) to evaluate in v0.2.

## Verified facts

_(Builder: Claude Code plugin/marketplace formats, Agent Skills spec, with dates and sources.)_
