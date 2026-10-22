// Recursive policy tests — build CLI values only; never invoke handlers.
// Test recursive policy — chỉ tạo CLI value, không gọi handler.
#![allow(clippy::unwrap_used)]

mod filter_commands;
mod supported_commands;
mod unsupported_recursive;

use clap::Parser;

fn parse_command(
    name: &str,
    needs_package: bool,
    needs_script: bool,
) -> crate::commands::definitions::Commands {
    let mut arguments = vec!["mgc", "--recursive", name];
    if needs_package {
        arguments.push("fixture-package");
    }
    if needs_script {
        arguments.push("fixture-script");
    }

    crate::Cli::try_parse_from(arguments)
        .unwrap_or_else(|error| panic!("CLI command '{name}' did not parse: {error}"))
        .command
        .unwrap_or_else(|| panic!("CLI command '{name}' produced no command"))
}
