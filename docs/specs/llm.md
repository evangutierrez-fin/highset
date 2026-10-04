# Spec: LLM provider layer and internal tasks (`highset-llm`, daemon `services/llm`)

Requirements: LLM-1, LLM-2, CTX-11, CTX-6, MTH-1. ADRs: 0019.

HighSet is an orchestrator, not an agent (ADR-0003). Direct model calls are reserved for small **internal tasks**. They never run on the request path of a user action that needs to be instant; they always run in the background with a visible status.

## 1. Providers

```rust
#[async_trait::async_trait]
pub trait LlmProvider: Send + Sync {
    fn id(&self) -> &str;                                   // "anthropic", "openai", "openai-compatible:<name>", "agent:<kind>"
    async fn complete(&self, req: LlmRequest) -> Result<LlmResponse, LlmError>;
}
pub struct LlmRequest { pub model: String, pub system: String, pub messages: Vec<Msg>, pub max_tokens: u32,
                        pub json_schema: Option<serde_json::Value>, pub timeout: Duration }
pub struct LlmResponse { pub text: String, pub json: Option<serde_json::Value>, pub usage: Usage, pub stop_reason: String }
```

| Provider | Transport | Notes |
|---|---|---|
| `anthropic` | Raw HTTPS (`reqwest` + rustls) to the Messages API (`POST /v1/messages`). There is no official Rust SDK. | Follow the official API docs at implementation time (headers, version, request shape). Check `stop_reason`, including `refusal`, before reading content. Use streaming for long outputs. For JSON outputs, use structured outputs (`output_config.format` with a JSON schema) instead of prompt-only JSON. Model IDs come from config and are never hardcoded in code. |
| `openai` | Raw HTTPS to OpenAI's API | Verify the current endpoint (Responses vs Chat Completions) in D-002 |
| `openai-compatible` | Same client, configurable `base_url` | For Ollama, LM Studio and other local servers |
| `agent:<kind>` | Runs an installed agent CLI in headless mode (`agents.md` §3.3) | Lets users with a subscription and no API key use internal tasks. Slower; used as a fallback |

- **Credentials:** `secret:anthropic_api_key`, `secret:openai_api_key` from the keychain. The env vars `ANTHROPIC_API_KEY` / `OPENAI_API_KEY` are accepted as a fallback and are never stored.
- **Retries:** 429 and 5xx with exponential backoff (max 3); timeouts default to 120 s.
- Every call records usage (`costs.md`).

## 2. Routing

```toml
[llm.tasks]
summarize_session = "anthropic:claude-haiku-4-5"
extract_memory    = "anthropic:claude-haiku-4-5"
tag_knowledge     = "anthropic:claude-haiku-4-5"
idea_to_spec      = "anthropic:claude-opus-5-5"
fallback          = "agent:claude-code"

[llm.providers.lmstudio]
kind = "openai-compatible"
base_url = "http://localhost:1234/v1"
```

- The defaults use a fast, cheap model for summarize/extract/tag (the owner accepted this in Q2.4) and the default Opus model for idea → spec. All of it is configurable.
- Resolution order: the configured route; if its provider has no credentials, `fallback`; if nothing is available, the task is skipped with an info message ("Configure an API key or install Claude Code to enable summaries").
- The builder verifies the model IDs against the provider's model list at implementation and release time.

## 3. Internal tasks (LLM-2)

| Task | Trigger | Input | Output | Failure behavior |
|---|---|---|---|---|
| `summarize_session` | Session end | Transcript (truncated smartly: first and last messages, tool-call titles, final diff stats, ≤ 30k tokens) | `summary.md`: ≤ 300 words; sections *What was done*, *Files changed*, *Open issues* | Write a placeholder summary from diff stats; never block |
| `extract_memory` | Session end (after summary) | Summary + transcript excerpts | JSON (schema): `[{type: decision\|learning\|todo, text, evidence}]` → inbox proposals (max 5) | Skip |
| `tag_knowledge` | KB ingest, if `auto_tag` | Title + first 2k tokens | JSON `{tags: [..≤5], kind_suggestion}` | Skip |
| `idea_to_spec` | `highset task spec T-n --draft` or the TUI action | Task + mentions + spec template + project overview | Draft `spec.md` in the task's spec dir (marked DRAFT) | Show the error; the user can retry |

- **Prompts** are templates in `content/templates/llm/*.md` (shipped defaults, overridable). They are versioned with a `prompt_version` frontmatter field recorded in outputs.
- Inputs and outputs are **redacted** (`security.md`).
- Tests use recorded fixtures (`wiremock`); CI never calls real APIs.

**Not included** (LLM-2): automatic naming of branches, commits or tasks. Branch names and commit messages come from deterministic templates (`git.md`).

## Verified facts

_(Builder: API versions and headers confirmed, model IDs confirmed, dates.)_
