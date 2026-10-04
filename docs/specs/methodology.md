# Spec: methodology (`highset-method`, daemon `services/method`)

Requirements: MTH-1…8, CTX-12, ORG-4. ADRs: 0013, 0018.

## 1. Flows

Flows are TOML files. The built-in `feature` and `fix` flows are embedded in the binary; users override or add flows in `content/flows/` or `.highset/flows/`.

```toml
# feature.toml (built-in)
id = "feature"
name = "Feature"
strictness = "guide"                       # guide | strict | info (project config can override)
statuses = ["inbox", "spec", "plan", "in_progress", "review", "done", "canceled"]

[[phase]]
id = "spec"
status = "spec"
profile = "planner"
artifacts = ["{spec_dir}/spec.md"]
template = "spec.md"
exit_gate = "approve_spec"
checkpoint_commit = true

[[phase]]
id = "plan"
status = "plan"
profile = "planner"
artifacts = ["{spec_dir}/plan.md", "{spec_dir}/tasks.md"]
template = ["plan.md", "tasks.md"]
exit_gate = "approve_plan"
checkpoint_commit = true

[[phase]]
id = "implement"
status = "in_progress"
profile = "builder"
checkpoint_commit = true

[[phase]]
id = "verify"
status = "review"
checks = ["verify:test", "verify:lint", "verify:format", "acceptance", "review_agent"]
review_profile = "reviewer"
exit_gate = "approve_diff"
on_exit = "finish"                          # finish = merge or PR per project config
```

`fix.toml` is the same without the `spec` phase. Its plan artifact is a single `plan.md` with a short "Problem / Fix / Test" structure.

## 2. Spec artifacts (Spec Kit–compatible)

- The spec directory per task is `<specs.dir>/<NNN>-<slug>/`, where NNN is the task number without the `T-` prefix, zero-padded to 3 digits. The default `specs.dir = "specs"` matches GitHub Spec Kit's layout: `specs/<NNN-feature>/spec.md`, `plan.md`, `tasks.md`.
- **Templates** follow Spec Kit's section structure (user scenarios, functional requirements, acceptance criteria, key entities, open questions / technical context, phases, tasks). Verify the current Spec Kit templates in D-004 and adapt section names. Record this under Verified facts.
- If a project already uses Spec Kit (`.specify/` present), HighSet reads its templates and its constitution (`.specify/memory/constitution.md`) as an always-on instruction source. It does not duplicate them.

## 3. Gates (MTH-2)

| Gate | Approves | Shown with |
|---|---|---|
| `approve_spec` | The spec | Spec rendered + open questions |
| `approve_plan` | Plan and tasks | Plan + task list + estimated risk |
| `approve_diff` | The final diff | Diff viewer + check results + reviewer verdict |

- Gates are approved or rejected from the TUI (`r` on the task, or the attention item) or with `highset gate approve|reject`.
- Rejecting requires a reason. The task goes back to the previous phase status, and the reason is attached to the next session's initial prompt.
- **Strictness:**
  - `guide` (default): moving past an unapproved gate is allowed with a `skip_reason`, which is logged.
  - `strict`: blocked.
  - `info`: gates are shown but never block.

## 4. Checks (MTH-3)

The `verify` phase runs the checks in order and records the results in the task's History and in the session timeline:

1. `verify:test`, `verify:lint`, `verify:format`, `verify:typecheck`: the project's `[verify]` commands, run in the worktree with a timeout (default 20 min). Output is captured and redacted.
2. `acceptance`: extracts the acceptance-criteria checklist from `spec.md` (or the task body for fixes). The reviewer agent marks each item, citing evidence (a test name, a file).
3. `review_agent`: a session with the `reviewer` profile. Its input is the diff, the spec/plan and the check outputs. Its output is a verdict (`PASS` / `CHANGES`) with findings.

`approve_diff` can only be approved when all checks pass, unless the owner overrides with a reason (logged).

## 5. Execution commands

| Command | Effect |
|---|---|
| `task start T-n` | Ensure the worktree; choose the phase for the current status; start a session with the phase profile and the compiled context, with the phase template as the initial instruction |
| `task advance T-n` | If the current phase's artifacts exist and its gate is approved (or skipped), move to the next phase (checkpoint commit) and start it |
| `task next T-n` | Explain the suggested next step in one sentence plus the command/key to run it (guide mode) |
| `task done T-n` | After `approve_diff`: finish (PR or local merge, `git.md` §4), changelog entry, staleness check, worktree cleanup |

## 6. Templates (MTH-5)

Shipped and editable: `spec.md`, `plan.md`, `tasks.md`, `adr.md` (MADR), `pr.md`, `retro.md`, plus the LLM task prompts (`llm.md`). Override order: project `.highset/templates/` > workspace > global `content/templates/` > built-in.

Variables available: `{{task.number}}`, `{{task.title}}`, `{{task.body}}`, `{{project.name}}`, `{{spec_dir}}`, `{{date}}`, `{{acceptance}}`. Use a minimal template engine (e.g. `minijinja`) with strict undefined-variable errors.

## 7. Logs (MTH-7)

- **ADRs:**
  - `highset adr new "<title>"` creates `docs/decisions/NNNN-<slug>.md` from the MADR template; numbering is max + 1.
  - Approved `decision` proposals produce an ADR with `status: accepted` and a link to the session.
  - Configurable location (`[method] adr_dir`, default `docs/decisions`).
- **Changelog:** on `task done`, add an entry under `## [Unreleased]` in `CHANGELOG.md` (Keep a Changelog). The section comes from the commit type: feat → Added, fix → Fixed, refactor/perf → Changed, docs → not logged unless configured. The text is the task title plus `(T-0042)`.
- **Journal:** `content/journal/YYYY/MM/DD.md`, appended by the daemon on task moves, session ends (with a one-line summary), gate decisions and approvals. Entries are grouped under the project and task headings.

## 8. Rituals (MTH-6)

- **Standup** (`highset standup`, or shown by the TUI on the first open of the day). It covers everything since the last standup (or 24 h):
  - Done: tasks moved to done; merged PRs.
  - Agents: sessions finished, with one-line summaries and costs.
  - Waiting for you: pending gates, attention items, inbox proposals.
  - Blocked: blocked tasks with reasons.
  - Suggested next: the top 3 tasks by priority whose dependencies are done.

  It is rendered as Markdown and saved into the journal.
- **Weekly review** (`highset review weekly`, suggested every Monday). It covers:
  - progress per project (moved/done counts);
  - costs per project and agent;
  - approved learnings;
  - stale-doc proposals;
  - tasks stuck in one status for more than 7 days.

## Verified facts

_(Builder: current Spec Kit layout and template sections, MADR template version, with dates.)_
