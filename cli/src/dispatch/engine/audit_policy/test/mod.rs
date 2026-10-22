// Audit-strict policy tests — keep command classification evidence beside its owner.
// Test policy audit-strict được đặt cạnh owner phân loại command.
#![allow(clippy::unwrap_used)]

mod rejected_commands;
mod supported_commands;

use clap::Parser;

fn parse_command(name: &str, needs_package: bool) -> crate::commands::definitions::Commands {
    let mut arguments = vec!["mgc", "--audit-strict", name];
    if needs_package {
        arguments.push("fixture-package");
    }

    crate::Cli::try_parse_from(arguments)
        .unwrap_or_else(|error| panic!("CLI command '{name}' did not parse: {error}"))
        .command
        .unwrap_or_else(|| panic!("CLI command '{name}' produced no command"))
}
