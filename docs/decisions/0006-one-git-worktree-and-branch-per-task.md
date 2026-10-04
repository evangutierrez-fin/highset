# 0006. One git worktree and branch per task

- Status: Accepted
- Date: 2026-10-03
- Source: questionnaire Q3.3, Q8.7 (`docs/inputs/questionnaire-2026-10-03.md`)

## Context

Several agents must work in parallel on the same repository without stepping on each other (P5).

## Decision

Each task gets its own git worktree (default under `~/.highset/worktrees/<project>/<T-id>-<slug>`) on a branch `hs/<T-id>-<slug>`, created when work starts. Checkpoint commits at phase transitions use Conventional Commits. Finishing a task creates a PR (via `gh`) or a local `--no-ff` merge. Agents never edit `.highset/` in worktrees; they use the MCP server for task state. Details in `specs/git.md`.

## Consequences

+ Isolation with a standard git feature; reviewable branches; parallelism for free.
- Disk usage per worktree; build caches are not shared (acceptable; documented).

## Alternatives considered

Single checkout with sequential agents (no parallelism). Containers per task (heavy, slow; may be added later as an option).
