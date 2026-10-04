---
id: A-001
title: "Git service: worktrees, branches, commits, diffs"
lane: agents
milestone: M2
size: M
status: todo
depends_on: [P1-005]
requirements: [AGT-3, MTH-8]
open_questions: []
---

# A-001 · Git service: worktrees, branches, commits, diffs

**Lane:** agents · **Milestone:** M2 · **Size:** M · **Depends on:** [P1-005](P1-005-daemon-skeleton-and-client-autostart.md)

## Goal

Safe, isolated workspaces per task.

## Read first

- [`specs/git.md`](../specs/git.md)
- ADR [0006](../decisions/0006-one-git-worktree-and-branch-per-task.md)
- PRD requirements: AGT-3, MTH-8

## Scope

- `highset-git` per git.md §1–3 and §5: create/reuse/remove worktrees, branch naming, base branch detection, checkpoint commits with Conventional Commit messages, diffs (file list + unified, paged), per-repo serialization, `worktree prune` on start.
- Daemon `services/git` with `git.diff`; every git command and exit status logged to the timeline (stderr redacted once D-001 exists).

## Acceptance criteria

- [ ] Integration tests on temp repos: create worktree + branch, commit, diff (committed + uncommitted), remove; removing a dirty worktree is refused without `force`.
- [ ] Paths with spaces and Unicode titles work.
- [ ] No command runs through a shell: a test uses a task title containing `;` and `$()`.
- [ ] The commit type mapping of git.md §2 is unit-tested.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
