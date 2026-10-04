---
id: R-002
title: "Packaging and release pipeline"
lane: release
milestone: M5
size: M
status: todo
depends_on: [P0-002, R-001]
requirements: [NFR-OSS-1, PERF-10]
open_questions: []
---

# R-002 · Packaging and release pipeline

**Lane:** release · **Milestone:** M5 · **Size:** M · **Depends on:** [P0-002](P0-002-ci-pipeline.md), [R-001](R-001-performance-verification.md)

## Goal

Install HighSet with one command.

## Read first

- [`ROADMAP.md#m5-v010--2026-12-01`](../ROADMAP.md#m5-v010--2026-12-01)
- ADR [0023](../decisions/0023-license.md)
- PRD requirements: NFR-OSS-1, PERF-10

## Scope

- cargo-dist (`dist`) for macOS arm64/x86_64 and Linux x86_64/arm64; GitHub Releases on tag; Homebrew tap formula; `cargo install` docs; SemVer; release checklist.
- Repository `github.com/evangutierrez-fin/highset`; Homebrew tap `github.com/evangutierrez-fin/homebrew-tap` (create it in this task).
- Refresh `pricing.toml` defaults and LLM model IDs before tagging.

## Acceptance criteria

- [ ] A dry-run release builds every artifact.
- [ ] Manual `brew install` on a clean macOS user is documented.
- [ ] Binary size ≤ 40 MB.
- [ ] Definition of Done in `AGENTS.md` satisfied.

## Verification log

_Fill in when finishing: for each acceptance criterion, the test name, command output, measurement or dated manual check that proves it._
