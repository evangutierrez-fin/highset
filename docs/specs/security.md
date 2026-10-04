# Spec: security and privacy (`highset-secrets`, cross-cutting)

Requirements: DATA-2/3/6, HRN-5, PLG-3, NFR-SEC-1. ADRs: 0016.

## 1. Threat model (v0.1)

**Assets:**

- API keys and tokens;
- private knowledge (local/audience-restricted items);
- source code;
- transcripts;
- the user's agent configurations.

| Threat | Mitigation |
|---|---|
| Another local user talks to the daemon | Socket `0600` in a `0700` dir owned by the user; peer-credential check (same UID) where the platform supports it |
| An agent (or a prompt injection it reads) exfiltrates private knowledge | Audience/level filtering at every boundary (compiler, MCP, mentions); MCP tokens bound to one session; unscoped MCP mode is read-only and returns unrestricted items only |
| An agent escalates (approves its own gates, merges, pushes) | Actor-aware transitions in core; gates are human-only; `git push` is denied by default profiles; HighSet only pushes on an explicit `task done --merge pr` |
| Secrets leak into files, transcripts, logs or generated configs | Keychain-only storage; launch-time env injection; redaction before persist; tests with canary secrets |
| A malicious plugin | Permission manifest, consent at install, re-consent on change, refuse undeclared executables/secrets; no auto-enabled Claude Code hooks |
| A destructive git operation | Never force-push, reset or clean the main checkout; refuse to remove dirty worktrees without force |
| Supply chain | `cargo deny` (advisories, licenses, bans, sources) in CI; pinned toolchain; minimal dependencies |

## 2. Secrets

- **Store:** the `keyring` crate (macOS Keychain, Linux Secret Service). Service name `highset`, account = secret name.
- **CLI:** `highset secret set <name>` reads the value from a hidden prompt or stdin, **never** from argv. `list` shows names and creation dates only. There is no command or RPC that returns a value.
- **References:** `secret:<name>` in profiles, MCP registry entries and LLM config. They are resolved only inside the daemon, at the moment of launching a process or making an HTTP call. They are passed as env vars to child processes and never written to disk.
- **Linux without Secret Service** (headless servers): fall back to an encrypted file only if the user opts in (`[secrets] backend = "file"` with a passphrase prompt). Otherwise env vars only. Document this; v0.1 may ship env-only as the fallback.

## 3. Redaction (DATA-3)

`Redactor::redact(&str) -> Cow<str>` is applied to:

- transcripts and summaries;
- knowledge ingests;
- proposals;
- MCP outputs;
- hook and check outputs;
- git stderr;
- log fields marked sensitive.

Detection:

1. **Exact values** of every secret in the keychain that HighSet manages (cached in memory as hashes and prefixes; matched with Aho-Corasick).
2. **Built-in patterns:**
   - Anthropic keys (`sk-ant-…`), OpenAI keys (`sk-…`, `sk-proj-…`);
   - GitHub tokens (`ghp_`, `gho_`, `github_pat_`), Slack tokens (`xox[abposr]-`), AWS access key IDs (`AKIA…`, `ASIA…`), Google API keys (`AIza…`);
   - PEM private key blocks, JWTs (three base64url segments with a valid JSON header);
   - generic `password=`/`token=`/`secret=` assignments with high-entropy values.
3. An optional high-entropy detector for long base64/hex strings (off by default to avoid false positives in code).

Replacement: `[REDACTED:<kind>:<first4>…]`. Redaction is idempotent and fast (≤ 1 ms per 64 KiB). Tests include positive and negative cases, plus a canary end-to-end test: a fake agent prints a stored secret, and the transcript must not contain it.

## 4. Permissions and sandboxing

- HighSet relies on each agent's **native sandbox** and compiles centralized rules into each agent's format (`harness.md` §5). In ACP mode it also enforces rules on permission requests and refuses fs requests outside the worktree.
- Generated configs deny edits to `.highset/**` and reads of `.env*` by default.
- **Hooks and plugin commands** run with the user's privileges. v0.1 provides consent and review, not isolation. OS-level sandboxing is evaluated in v0.2.

## 5. Network and privacy

- No telemetry, no update checks by default (`[updates] check = false`; Homebrew handles updates).
- Outbound connections happen only when the user triggers: URL ingest, LLM tasks (configured), plugin install/update, `gh` and git remotes, MCP servers the user registered.
- Logs stay local in `state/logs/`, rotate daily, keep 14 days, and never contain prompts or transcripts at `info` level.

## 6. Reporting

`SECURITY.md` (P0-003) explains private vulnerability reporting through GitHub Security Advisories.

## Verified facts

_(Builder: keyring backends verified on macOS and Linux CI; peer-cred support; patterns reviewed.)_
