// Identity tests — validate selectors and CLI values without dispatching handlers.
// Test identity — kiểm tra selector và CLI value, không chạy dispatch handler.
#![allow(clippy::unwrap_used)]

use super::identity::{
    explicit_dependency_core, resolve_recursive_workspace_core, validate_explicit_dependency_core,
    workspace_package_name,
};

mod command_routes;
mod explicit_dependency;
mod workspace_identity;
mod workspace_package_name;

fn parse_command(name: &str, needs_package: bool) -> crate::Commands {
    use clap::Parser;

    let mut arguments = vec!["mgc", name];
    if needs_package {
        arguments.push("fixture-package");
    }

    crate::Cli::try_parse_from(arguments)
        .unwrap_or_else(|error| panic!("public command '{name}' did not parse: {error}"))
        .command
        .unwrap_or_else(|| panic!("public command '{name}' produced no command"))
}
