# CLAUDE.md

@AGENTS.md

## Claude Code specifics

- Use the **`planner`** subagent (`.claude/agents/planner.md`) before any M/L task or contract change.
- Use the **`reviewer`** subagent (`.claude/agents/reviewer.md`) before you mark a task `done`, and fix every blocking finding it reports.
- Skills in `.claude/skills/`:
  - `highset-task`: run one backlog task end to end. This is your default loop.
  - `highset-adr`: record or supersede an architecture decision.
  - `phase-review`: write the Spanish milestone report for the owner and stop.
- When several Claude sessions work in parallel (after M1), each session owns one lane and works in its own git worktree. Don't touch another lane's crates.
- Keep messages to the owner short and in Spanish. Report what changed, what you verified, and what needs their decision.
