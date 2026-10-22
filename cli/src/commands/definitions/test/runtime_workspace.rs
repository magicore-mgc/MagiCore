// Runtime Workspace CLI definition tests.
// Test lược đồ CLI theo nhóm chức năng.
use crate::Cli;
use crate::commands::definitions::Commands;
use crate::commands::definitions::commands::{BareCommands, EngineCommands};
use clap::Parser;

#[test]
fn test_dev_command_accepts_host_and_port() {
    let cli = Cli::try_parse_from(["mgc", "dev", "--host", "127.0.0.1", "--port", "4315"]).unwrap();

    match cli.command.unwrap() {
        Commands::Engine(EngineCommands::Dev {
            host,
            port,
            clear: _,
            compat_runtime,
            compat_runner,
        }) => {
            assert_eq!(host.as_deref(), Some("127.0.0.1"));
            assert_eq!(port, Some(4315));
            assert!(compat_runtime.is_none());
            assert!(compat_runner.is_none());
        }
        _ => panic!("expected dev command"),
    }
}

#[test]
fn test_deploy_defaults_to_dry_run() {
    let cli = Cli::try_parse_from(["mgc", "deploy"]).unwrap();
    match cli.command.unwrap() {
        Commands::Engine(EngineCommands::Deploy { run }) => assert!(!run),
        _ => panic!("expected deploy command"),
    }
    let cli = Cli::try_parse_from(["mgc", "deploy", "--run"]).unwrap();
    match cli.command.unwrap() {
        Commands::Engine(EngineCommands::Deploy { run }) => assert!(run),
        _ => panic!("expected deploy command"),
    }
}

#[test]
fn test_workspace_list_parses_filter_and_json() {
    let cli = Cli::try_parse_from(["mgc", "workspace", "list", "--filter", "./apps/*", "--json"])
        .unwrap();

    match cli.command.unwrap() {
        Commands::Bare(BareCommands::Workspace { cmd }) => match cmd {
            crate::commands::workspace::WorkspaceCmd::List { filter, json } => {
                assert_eq!(filter.as_deref(), Some("./apps/*"));
                assert!(json);
            }
        },
        _ => panic!("expected workspace command"),
    }
}

#[test]
fn test_dev_accepts_compat_runner_flag() {
    let cli = Cli::try_parse_from(["mgc", "dev", "--compat-runner", "npm"]).unwrap();

    match cli.command.unwrap() {
        Commands::Engine(EngineCommands::Dev { compat_runner, .. }) => {
            assert_eq!(compat_runner.as_deref(), Some("npm"));
        }
        _ => panic!("expected dev command"),
    }
}
