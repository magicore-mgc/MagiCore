// Basic bare command mapping — preserve package and documentation values.
// Mapping lệnh bare cơ bản — giữ package và giá trị tài liệu.
use super::super::bare_common_to_common;
use crate::commands::completion::CompletionShell;
use crate::commands::definitions::commands::bare::BareCommands;
use crate::dispatch::types::CommonCommand;
use std::path::PathBuf;

fn mapped(command: BareCommands) -> CommonCommand {
    match bare_common_to_common(command) {
        Some(command) => command,
        None => panic!("common bare command must be mapped"),
    }
}

#[test]
fn maps_link_package() {
    match mapped(BareCommands::Link {
        package: Some("@example/shared".to_owned()),
    }) {
        CommonCommand::Link { package } => {
            assert_eq!(package.as_deref(), Some("@example/shared"));
        }
        _ => panic!("link command mapped to a different variant"),
    }
}

#[test]
fn maps_unlink_package() {
    match mapped(BareCommands::Unlink {
        package: Some("@example/shared".to_owned()),
    }) {
        CommonCommand::Unlink { package } => {
            assert_eq!(package.as_deref(), Some("@example/shared"));
        }
        _ => panic!("unlink command mapped to a different variant"),
    }
}

#[test]
fn maps_why_package_name() {
    match mapped(BareCommands::Why {
        package: "react".to_owned(),
    }) {
        CommonCommand::Why { package } => assert_eq!(package, "react"),
        _ => panic!("why command mapped to a different variant"),
    }
}

#[test]
fn maps_docs_output_path() {
    let output = PathBuf::from("docs/commands.md");
    match mapped(BareCommands::Docs {
        output: Some(output.clone()),
    }) {
        CommonCommand::Docs { output: actual } => assert_eq!(actual, Some(output)),
        _ => panic!("docs command mapped to a different variant"),
    }
}

#[test]
fn maps_completion_shell() {
    assert!(matches!(
        mapped(BareCommands::Completion {
            shell: CompletionShell::Zsh,
        }),
        CommonCommand::Completion {
            shell: CompletionShell::Zsh
        }
    ));
}
