---
name: highset-adr
description: Record a new architecture decision or supersede an existing one in docs/decisions (MADR format). Use when a task needs to deviate from an ADR or spec, when choosing between technical options with lasting impact (e.g. a config injection strategy, a crate choice that shapes the design), or when a breaking contract change is needed.
---

# Record an architecture decision

1. **Check first.** Read `docs/decisions/README.md`. If an existing ADR already covers the topic, you are superseding it, not adding a parallel one.
2. **Number.** Use the highest existing number + 1, zero-padded to 4 digits. Filename: `NNNN-<short-kebab-title>.md`.
3. **Write it** from `docs/decisions/0000-template.md`:
   - **Status:** `Proposed`.
   - **Source:** the task ID, plus the questionnaire references (`docs/inputs/questionnaire-2026-10-03.md`) if the owner's intent is involved.
   - **Context:** the forces at play, with facts you verified (links).
   - **Decision:** stated so another agent can follow it without asking.
   - **Consequences:** both good (+) and bad (-).
   - **Alternatives considered:** with the reason each was rejected.
4. **Decide who approves.**
   - If the decision changes owner-visible behavior, scope, timeline, or contradicts the owner's questionnaire answers: add a row to `docs/OPEN-QUESTIONS.md` and ask the owner in Spanish, briefly, with your recommendation. Leave the ADR `Proposed` until they answer.
   - Otherwise set `Accepted` and continue.
5. **If it supersedes an ADR,** edit only the old ADR's status line: `Superseded by NNNN`.
6. **Update** `docs/decisions/README.md` (the index table) and every spec section the decision affects.
7. **Commit** with `docs(adr): NNNN <title>` and a `Refs: <task-id>` footer.
