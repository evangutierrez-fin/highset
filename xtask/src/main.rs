//! Repository automation for HighSet. Run `cargo run -p xtask -- <command>`.
//!
//! Commands:
//! - `check-deps`: fail on internal dependency edges that ARCHITECTURE.md §2 forbids.
#![forbid(unsafe_code)]

mod check_deps;

use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

const USAGE: &str = "usage: cargo run -p xtask -- <command>

commands:
  check-deps   check internal crate dependencies against docs/ARCHITECTURE.md §2";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let result = match args.first().map(String::as_str) {
        Some("check-deps") => run_check_deps(),
        Some("-h" | "--help") => {
            println!("{USAGE}");
            Ok(())
        }
        Some(other) => Err(format!("unknown command `{other}`\n\n{USAGE}")),
        None => Err(USAGE.to_owned()),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}

/// The workspace root: the parent of this crate's directory.
///
/// `cargo run` sets `CARGO_MANIFEST_DIR` at run time; prefer it over the compile-time value, so a
/// binary reused from a shared `CARGO_TARGET_DIR` still checks the checkout it was run from.
fn workspace_root() -> PathBuf {
    let manifest_dir = std::env::var_os("CARGO_MANIFEST_DIR")
        .map_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")), PathBuf::from);
    manifest_dir
        .parent()
        .map_or_else(|| manifest_dir.clone(), Path::to_path_buf)
}

fn run_check_deps() -> Result<(), String> {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned());
    let root = workspace_root();
    let output = Command::new(cargo)
        .args([
            "metadata",
            "--format-version",
            "1",
            "--no-deps",
            "--offline",
        ])
        .current_dir(&root)
        .output()
        .map_err(|e| format!("could not run `cargo metadata`: {e}"))?;
    if !output.status.success() {
        return Err(format!(
            "`cargo metadata` failed:\n{}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    let metadata: check_deps::Metadata = serde_json::from_slice(&output.stdout)
        .map_err(|e| format!("could not parse `cargo metadata` output: {e}"))?;

    let violations = check_deps::check(&metadata);
    if violations.is_empty() {
        println!(
            "check-deps: all internal dependency edges are allowed ({})",
            root.display()
        );
        return Ok(());
    }
    let mut lines = vec![format!(
        "check-deps: {} forbidden edge(s):",
        violations.len()
    )];
    lines.extend(violations.iter().map(|v| format!("  {v}")));
    lines.push("See docs/ARCHITECTURE.md §2 \"Dependency rules\".".to_owned());
    Err(lines.join("\n"))
}
