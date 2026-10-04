//! The `highset` binary: the clap command tree and dispatch to the TUI, the daemon, the CLI
//! commands and the MCP server.
//!
//! See this crate's `README.md` for its lane and allowed internal dependencies.
#![forbid(unsafe_code)]

use clap::Command;

/// The binary name users type.
pub const BIN_NAME: &str = "highset";

/// Builds the top-level command tree.
///
/// Subcommands are added by later tasks; each lane adds its own module under `src/commands/`.
pub fn command() -> Command {
    Command::new(BIN_NAME)
        .version(env!("CARGO_PKG_VERSION"))
        .about("A command center for working with AI coding agents, from your terminal.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_tree_is_valid() {
        command().debug_assert();
    }

    #[test]
    fn version_flag_prints_name_and_version() {
        let out = command().render_version();
        assert_eq!(out, format!("highset {}\n", env!("CARGO_PKG_VERSION")));
    }
}
