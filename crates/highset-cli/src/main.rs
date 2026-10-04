//! Entry point of the `highset` binary.
#![forbid(unsafe_code)]

use std::process::ExitCode;

fn main() -> ExitCode {
    let _matches = highset_cli::command().get_matches();
    ExitCode::SUCCESS
}
