# Spec: costs and usage (daemon `services/costs`)

Requirements: COST-1…4, ORG-5. ADRs: none specific.

## 1. Sources of usage

| Source | When |
|---|---|
| ACP / headless events | When the agent reports token usage in its stream |
| Transcript harvesters | PTY sessions; also to fill gaps (`agents.md` §6) |
| Internal LLM tasks | Every `highset-llm` call records usage with `source = llm_task` |

Records go to `usage.jsonl` in the session dir and to the `usage` table. Duplicate reports (event + harvester) are deduplicated by `(session, message id or timestamp window)`.

## 2. Pricing

`content/pricing.toml`: shipped defaults, copied on first run, user-editable.

```toml
last_verified = "2026-10-03"      # builder updates this when refreshing prices
currency = "USD"

[models."anthropic:claude-opus-5-5"]
input_per_mtok = 4.00
output_per_mtok = 20.00
cache_read_per_mtok = 0.20

[models."anthropic:claude-haiku-4-5"]
input_per_mtok = 1.00
output_per_mtok = 5.00
```

- Unknown models → cost `null`, tokens still counted. The UI shows "price unknown" with a hint to edit `pricing.toml`.
- The builder must refresh the defaults from official pricing pages at release time (R-002) and update `last_verified`.

## 3. Billing modes (COST-3)

`[agents.<kind>] billing = "api" | "subscription"`. Default `subscription` for Claude Code, Codex and Cursor (CLIs typically used with a plan); `api` for internal LLM tasks with API keys. In subscription mode the UI shows tokens and request counts, never money, and budgets are token-based.

## 4. Aggregates

Per session, task, project, workspace, agent, model and day, computed in SQLite on demand (indexed by `session_id`, `at`). The task's `cost` field in the UI is the sum over its sessions.

## 5. Budgets (COST-4)

```toml
# project.toml or task frontmatter (config layer)
[[costs.budget]]
period = "month"          # month | week | total
limit_usd = 50            # or limit_tokens = 20_000_000
on_exceed = "alert"       # alert | pause
```

- **Alerts** fire at 80% and 100%. Each threshold fires once per period, as a `budget_alert` attention item plus a notification.
- `pause` at 100%: running sessions in scope get a cancel (ACP) or SIGINT (PTY). New sessions in scope are refused with `budget_exceeded`, unless `--override-budget` is given with a reason (logged).

## 6. Surfaces

- `highset cost [--project] [--task] [--since 7d] [--by agent|model|day]`
- TUI: status bar shows today's total; the dashboard shows per-project and per-agent breakdowns; the task detail shows cost and tokens.
- The weekly review includes costs (`methodology.md` §8).

## Verified facts

_(Builder: which agents report usage in which mode; price sources and dates.)_
