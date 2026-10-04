# Spec: git integration (`highset-git`, daemon `services/git`)

Requirements: AGT-3, MTH-8, INT-1. ADRs: 0006.

All git operations shell out to the `git` CLI (≥ 2.38) using `Command` with explicit arguments, never `sh -c`. Operations on the same repository are serialized per repo; different repos run concurrently.

## 1. Worktrees and branches

- A task gets a worktree the first time a session starts on it (or on `task start`).
- **Path:** `<worktrees_dir>/<project-slug>/<T-number>-<slug>/`. `slug` is the title, lowercased, ASCII-folded, max 40 chars.
- **Branch:** `<branch_prefix><T-number>-<slug>`, default prefix `hs/`, e.g. `hs/T-0042-add-login`.
- **Base:** the project's `default_branch` (detected from `origin/HEAD`, else `main`/`master`). It can be overridden per task.
- **Command:** `git -C <root> worktree add -b <branch> <path> <base>`. If the branch already exists (resumed task), use `git worktree add <path> <branch>`.
- After creation:
  1. Mark `.highset/` as read-only for agents in the permissions (enforced by sync targets).
  2. Add generated, uncommitted sync outputs to the worktree's `info/exclude` when `sync.commit_generated = false`.
- **Removal** (`task done` or `task rm`): refuse if the worktree has uncommitted changes, unless `force` is set. Then run `git worktree remove` and delete the branch if it is merged (or keep it, depending on `git.delete_merged_branches`, default true).
- `git worktree prune` runs on daemon start.

## 2. Checkpoint commits (Q8.7)

- When a flow phase completes (spec → plan → implement → verify), HighSet commits all changes in the worktree if there are any.
- **Message (Conventional Commits),** with no LLM involved (LLM-2):

  ```
  <type>(<scope>): <task title> [<phase>]

  Refs: T-0042
  HighSet-Session: ses_01JB…
  ```

- `type` comes from the flow and phase: feature → `feat`, fix → `fix`, spec/plan-only commits → `docs`, documenter profile → `docs`, otherwise `chore`. `scope` is the task's `epic` or first label, or is omitted.
- Agents may also commit on their own. HighSet never rewrites agent commits.

## 3. Diffs

- `git.diff {task}` returns the unified diff of `merge-base(base, HEAD)..HEAD` plus uncommitted changes, with per-file stats.
- Large diffs are paged by file. The TUI requests the file list first, then individual files.
- `diff.patch` in the session record is written at session end, from the same computation.

## 4. Finishing a task

`task done` runs after the `approve_diff` gate and the checks (`methodology.md`).

- `--merge pr` (default when the remote is GitHub and `gh` is available):
  1. Push the branch with `git push -u origin <branch>`. This is the only push HighSet performs, and only on this explicit action.
  2. Run `gh pr create --title "<conventional title>" --body-file <rendered PR template>`, adding `--draft` if requested.
  3. Store the PR URL in the task's `links`.
- `--merge local`: `git -C <root> merge --no-ff <branch>` into the base branch in the main checkout. Refuse if the main checkout is dirty.
- If `gh` is missing, print the exact commands to run and leave the task in `review` with a note.

## 5. Safety

- Never run `git push --force`, `reset --hard` or `clean -fdx` on the main checkout.
- Never delete branches that are not merged, unless `--force` is given with explicit confirmation in the TUI.
- Every git command and its exit status goes to the session timeline. Stderr is redacted.

## Verified facts

_(Builder: git version requirements confirmed, `gh` flags confirmed, with dates.)_
