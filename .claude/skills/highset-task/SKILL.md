---
name: highset-task
description: Run one HighSet backlog task end to end (pick, claim, plan, verify external facts, implement with tests, check, review, finish). Use when asked to work on the HighSet build, to continue the backlog, or to do a specific task ID like P1-003 or B-006.
---

# Run one HighSet backlog task

Follow these steps in order. `AGENTS.md` is the authority on conventions; this skill is the procedure.

## 1. Pick

- If you were given a task ID, open `docs/tasks/<ID>-*.md`.
- Otherwise open `docs/tasks/README.md` and take the first `todo` task in **your lane** whose `depends_on` tasks are all `done`. If no lane was assigned: during Phases 0–1, use lanes `infra` then `core`; after M1, ask the owner which lane this session owns.
- Check the task's `open_questions`. If one blocks the work, do the unblocked parts and note the rest; never guess an owner decision.

## 2. Claim

```bash
git switch -c task/<ID>-<short-slug>          # or: git worktree add ../highset-<ID> -b task/<ID>-<short-slug>
```

Set `status: in-progress` in the task file and on the board, then commit: `chore(tasks): start <ID>` with `Refs: <ID>`.

## 3. Plan

For size M/L, or any contract change, run the **planner** subagent with the task ID and read its plan. For size S, write a 5–10 line plan yourself in your notes. Resolve any "External facts to verify" before coding: read the official docs or source, then record the facts with date and source in the spec's **Verified facts** section.

## 4. Implement

- Write the tests named in the plan first, where practical.
- Work in small steps. Keep the workspace compiling, and commit with Conventional Commits plus a `Refs: <ID>` footer.
- Stay in your lane's crates. Additive contract changes get a note in the spec; breaking ones go through `highset-adr` and a separate change merged first.
- Write user-facing strings as Fluent keys in **en and es**.

## 5. Check

```bash
cargo fmt --all
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo run -p xtask -- check-deps
cargo run -p xtask -- check-i18n     # once it exists
```

Measure any performance budget the task names, and paste the numbers into the verification log.

## 6. Review

Run the **reviewer** subagent with the task ID. Fix every `blocking` finding and the `should-fix` ones unless you can justify skipping them, then re-run it until the verdict is `PASS`.

## 7. Finish

1. Fill the task's **Verification log**: one line per acceptance criterion with its evidence.
2. Set `status: done` (or `review` if the owner must check something manually; say what in the log). Update the board.
3. Add a `CHANGELOG.md` entry under *Unreleased* if the change is user-visible.
4. Update specs if the behavior differs from them.
5. Rebase on `main`, re-run step 5, then merge (fast-forward or merge commit) and remove the worktree.
6. If this was the last task of a milestone (see `docs/ROADMAP.md`), run the `phase-review` skill and stop.

## 8. Report

Tell the owner in Spanish, in 3–5 lines: what was done, how it was verified, and anything that needs their decision.
