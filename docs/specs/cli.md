# Spec: CLI (`highset-cli`)

Requirements: UX-10, ORG-6, INT-2. Every TUI action has a CLI equivalent. Every command accepts `--json` (machine output on stdout; human messages go to stderr) and `--project <path|slug>` (default: discovered from the current directory).

The binary is `highset`. Users may add their own shell alias (e.g. `alias hs=highset`); HighSet does not install one.

## Command tree

```
highset                                  open the TUI (same as `highset tui`)
highset add "<text>" [--project P]      quick capture to the Inbox (global inbox outside a project)

highset daemon run|start|stop|restart|status
highset doctor                           environment checks with fix hints
highset config show [--resolved] [--task T-n]
highset config set <key> <value> [--scope global|workspace|project|task]
highset reindex

highset ws list|add <slug> [--name]|rename <slug> <name>|rm <slug>
highset project list|add [path] [--ws W] [--name N]|show|link <repo-path>|rm

highset task list [--status S] [--label L] [--agent A] [--all-projects]
highset task add "<title>" [--flow feature|fix] [--priority P] [--label L]... [--epic E] [--dep T-n]...
highset task show T-n
highset task edit T-n                    opens $EDITOR, validates on save
highset task move T-n <status> [--skip-reason "…"]
highset task link T-n <url|path|commit>
highset task move-to T-n <project>      e.g. from the global inbox
highset task start T-n [--agent A] [--profile P] [--mode acp|pty|headless] [--prompt "… @kb:x"]
highset task next T-n                    suggested next step (guide mode)
highset task advance T-n                 run the next phase of the flow
highset task done T-n [--merge local|pr] [--draft]
highset gate approve|reject T-n <gate> [--note|--reason]

highset session list [--running] | show <id> | attach <id> | stop <id> | log <id> [--follow]
highset session start [--task T-n] --agent A [--profile P] [--mode M] [--prompt "…"]
highset attention list | approve <id> [--option O] | deny <id>
highset hook signal --kind needs_input|finished|permission   (called by agent-native hooks)

highset kb list [--kind K] [--level L] [--tag T] | add <path|url|-> [--kind K] [--level L] [--tag T] [--audience-agent A]...
highset kb show <slug> | edit <slug> | rm <slug> | mv <slug> --level L
highset context ls                       convention files found per level
highset context explain [T-n] [--agent A] [--profile P]
highset sync [--dry-run] [--target T]    write sync targets for the project (diff first)
highset pack list | new <name> | show <name> | enable <name> [--task T-n] | disable <name> [--task T-n]
highset profile list | show <name> [--resolved] [--task T-n] | new <name> | edit <name>
highset mcp-server list | add <name> -- <command> [args…] | rm <name> | enable <name> [--profile P] | disable <name>
highset mcp serve                        stdio MCP server (spawned by agents; not for humans)

highset inbox list | show <id> | approve <id> | edit <id> | reject <id> [--reason]
highset search "<query>" [--type task|kb|spec|transcript] [--project P]
highset cost [--project P] [--task T-n] [--since 7d]
highset budget set --project P|--task T-n (--usd N | --tokens N) [--period month|total] [--on-exceed alert|pause]
highset standup [--since <date>]
highset review weekly
highset adr new "<title>"

highset plugin list | inspect <source> | add <source> | update [id] | rm <id>
highset secret set <name> | list | rm <name>       (value read from a prompt or stdin, never from argv)
highset backup now | list | restore <date> [--dry-run]
highset agent list                       detected agents, versions, supported modes
```

## Conventions

- Exit codes: `0` ok, `1` error, `2` usage error, `3` daemon already running / not running, `4` gate required, `5` not found.
- Output is human-friendly and localized by default. `--json` is stable and documented by examples in this file as commands are built.
- Commands never prompt when stdout is not a TTY, except `secret set`, which reads stdin.
- `highset add` is optimized for latency (PERF-3). It needs no TUI or i18n heavyweights beyond the message catalog.

## Verified facts

_(Builder: add examples of `--json` output for each command as it is implemented.)_
