# 0023. License: MIT

- Status: Accepted
- Date: 2026-10-03
- Source: questionnaire Q1.5, Q12.7 (`docs/inputs/questionnaire-2026-10-03.md`); owner decision on OQ-1 (`docs/OPEN-QUESTIONS.md`)

## Context

The owner wants the project public and open source from day 1 (Q1.5), but first chose "private for now, decide later" for the license (Q12.7). A public repository without a license is not open source (all rights reserved by default), so the conflict had to be resolved before publishing.

## Decision

HighSet is licensed under the **MIT License**. The `LICENSE` file at the repository root holds the text, with the copyright line "HighSet contributors". Every crate declares `license = "MIT"` through `[workspace.package]`. Third-party dependencies must be compatible with MIT distribution; `cargo deny` enforces the allow-list in P0-002. Crates stay `publish = false` until R-002 decides whether to publish to crates.io.

## Consequences

+ Maximum adoption and compatibility with the Rust ecosystem; simple for contributors.
+ The repository can be made public at any time.
- No explicit patent grant (Apache-2.0 has one); accepted by the owner for simplicity.
- Contributions are accepted under the same license (stated in CONTRIBUTING.md, P0-003).

## Alternatives considered

- MIT OR Apache-2.0 dual license (the Rust convention, adds a patent grant): the owner chose MIT alone.
- Apache-2.0 only.
- AGPL (strong copyleft, limits adoption).
- Proprietary (contradicts Q1.5).
