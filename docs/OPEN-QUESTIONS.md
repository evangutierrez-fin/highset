# Open questions

Decisions that belong to the owner. Builder agents add items here instead of guessing, keep working on unblocked tasks, and reference the ID (e.g. `OQ-4`) in affected tasks. Ask the owner in Spanish.

| ID | Question | Blocks | Recommendation | Status |
|---|---|---|---|---|
| — | _No open questions._ | | | |

## Resolved

| ID | Question | Decision | Date | Effect |
|---|---|---|---|---|
| OQ-1 | License: "open source from day 1" (Q1.5) vs. "private for now" (Q12.7) | **MIT** | 2026-10-03 | ADR 0023 accepted; `LICENSE` added at the root; crates declare `license = "MIT"` (P0-001). |
| OQ-2 | MVP scope (all nine features, Q13.1) vs. the 4–6 week wish (Q1.8) | **8 weeks, full v0.1 scope** as defined in `PRD.md` §9 | 2026-10-03 | Roadmap dates stand (M5 ≈ 2026-12-01); no features moved to v0.2. |
| OQ-3 | Where to publish | GitHub account **`evangutierrez-fin`**: repo `github.com/evangutierrez-fin/highset`, Homebrew tap `github.com/evangutierrez-fin/homebrew-tap` | 2026-10-03 | Used in P0-001 (Cargo `repository`), P0-002 (badges), R-002 (releases, tap). The owner named the account by email; `evangutierrez-fin` is the account logged into `gh` on the owner's machine. |
| OQ-4 | GitHub repository visibility (needed for CI in P0-002) | **Public now** | 2026-10-03 | Repo created public; GitHub Actions minutes are free for public repos, including macOS runners. Matches Q1.5 (open source from day 1). |
