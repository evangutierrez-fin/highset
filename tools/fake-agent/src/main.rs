//! A test double for agent CLIs. It speaks ACP, PTY and headless JSON according to scripted
//! scenarios, so tests never call real agents.
//!
//! The modes are implemented in P0-006. See this crate's `README.md`.
#![forbid(unsafe_code)]

use std::process::ExitCode;

fn main() -> ExitCode {
    ExitCode::SUCCESS
}
