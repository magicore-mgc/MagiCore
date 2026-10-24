// Cli Surface CLI definition tests.
// Test lược đồ CLI theo nhóm chức năng.
use crate::Cli;
use crate::commands::definitions::Commands;
use crate::commands::definitions::commands::{AddCommands, CommonCommands, EngineCommands};
use clap::{CommandFactory, Parser};

#[test]
fn test_config_parses_get_set_delete_local() {
    let get = Cli::try_parse_from(["mgc", "config", "get", "registry"]).unwrap();
    match get.command.unwrap() {
        Commands::Common(CommonCommands::Config { cmd, local }) => match cmd {
            crate::commands::config::ConfigCmd::Get { key } => {
                assert_eq!(key, "registry");
                assert!(!local);
            }
            _ => panic!("expected config get"),
        },
        _ => panic!("expected config command"),
    }

    let set = Cli::try_parse_from(["mgc", "config", "set", "x", "1", "--local"]).unwrap();
    match set.command.unwrap() {
        Commands::Common(CommonCommands::Config { cmd, local }) => match cmd {
            crate::commands::config::ConfigCmd::Set { key, value, toml } => {
                assert_eq!(key, "x");
                assert_eq!(value, "1");
                assert!(local);
                assert!(!toml); // mặc định không phải toml
            }
            _ => panic!("expected config set"),
        },
        _ => panic!("expected config command"),
    }

    // mgc config set --toml
    let set_toml =
        Cli::try_parse_from(["mgc", "config", "set", "ecosystem", "web", "--toml"]).unwrap();
    match set_toml.command.unwrap() {
        Commands::Common(CommonCommands::Config { cmd, .. }) => match cmd {
            crate::commands::config::ConfigCmd::Set { key, value, toml } => {
                assert_eq!(key, "ecosystem");
                assert_eq!(value, "web");
                assert!(toml);
            }
            _ => panic!("expected config set --toml"),
        },
        _ => panic!("expected config command"),
    }

    // mgc config unset
    let unset = Cli::try_parse_from(["mgc", "config", "unset", "registry"]).unwrap();
    match unset.command.unwrap() {
        Commands::Common(CommonCommands::Config { cmd, .. }) => match cmd {
            crate::commands::config::ConfigCmd::Unset { key, .. } => {
                assert_eq!(key, "registry");
            }
            _ => panic!("expected config unset"),
        },
        _ => panic!("expected config command"),
    }

    // mgc config list --local
    let list = Cli::try_parse_from(["mgc", "config", "list", "--local"]).unwrap();
    match list.command.unwrap() {
        Commands::Common(CommonCommands::Config { cmd, .. }) => match cmd {
            crate::commands::config::ConfigCmd::List { local } => {
                assert!(local);
            }
            _ => panic!("expected config list"),
        },
        _ => panic!("expected config command"),
    }
}

#[test]
fn test_stage_parses_dir_flag() {
    let cli = Cli::try_parse_from(["mgc", "stage", "--dir", "/tmp/demo"]).unwrap();
    match cli.command.unwrap() {
        Commands::Common(CommonCommands::Stage { dir }) => {
            assert_eq!(dir.as_deref(), Some(std::path::Path::new("/tmp/demo")));
        }
        _ => panic!("expected stage command"),
    }
}

#[test]
fn test_per_core_aliases_resolve() {
    let cmd = Cli::command();
    for (alias, expected) in [
        ("i-web", "install-web"),
        ("a-lib", "add-lib"),
        ("rm-ai", "remove-ai"),
        ("up-clo", "update-clo"),
        ("ls-hardware", "list-hardware"),
        ("i-hardware", "install-hardware"),
    ] {
        let found = cmd
            .find_subcommand(alias)
            .unwrap_or_else(|| panic!("alias {alias} not found"));
        assert_eq!(
            found.get_name(),
            expected,
            "alias {alias} should map to {expected}"
        );
    }
}

#[test]
fn test_global_quiet_flag_parses() {
    let cli = Cli::try_parse_from(["mgc", "--quiet", "add-web", "zod"]).unwrap();
    assert!(cli.quiet);
    match cli.command.unwrap() {
        Commands::Add(AddCommands::AddWeb { packages, .. }) => assert_eq!(packages, vec!["zod"]),
        _ => panic!("expected add-web command"),
    }
}

#[test]
fn test_cache_command_accepts_status_and_clean_targets() {
    let status = Cli::try_parse_from(["mgc", "cache", "status", "--target", "shared"]).unwrap();
    match status.command.unwrap() {
        Commands::Engine(EngineCommands::Cache {
            action,
            target,
            yes,
            ..
        }) => {
            assert_eq!(action, "status");
            assert_eq!(target, "shared");
            assert!(!yes);
        }
        _ => panic!("expected cache command"),
    }

    let clean =
        Cli::try_parse_from(["mgc", "cache", "clean", "--target", "build", "--yes"]).unwrap();
    match clean.command.unwrap() {
        Commands::Engine(EngineCommands::Cache {
            action,
            target,
            yes,
            ..
        }) => {
            assert_eq!(action, "clean");
            assert_eq!(target, "build");
            assert!(yes);
        }
        _ => panic!("expected cache command"),
    }

    let prune =
        Cli::try_parse_from(["mgc", "cache", "prune", "--target", "shared", "--yes"]).unwrap();
    match prune.command.unwrap() {
        Commands::Engine(EngineCommands::Cache {
            action,
            target,
            yes,
            dry_run,
            ..
        }) => {
            assert_eq!(action, "prune");
            assert_eq!(target, "shared");
            assert!(yes);
            assert!(!dry_run);
        }
        _ => panic!("expected cache command"),
    }

    let dry_run =
        Cli::try_parse_from(["mgc", "cache", "prune", "--target", "shared", "--dry-run"]).unwrap();
    match dry_run.command.unwrap() {
        Commands::Engine(EngineCommands::Cache {
            action,
            target,
            yes,
            dry_run,
            ci,
        }) => {
            assert_eq!(action, "prune");
            assert_eq!(target, "shared");
            assert!(!yes);
            assert!(dry_run);
            assert!(!ci);
        }
        _ => panic!("expected cache command"),
    }
}

#[test]
fn test_help_surface_matches_build_shape() {
    let help = Cli::command().render_long_help().to_string();

    assert!(help.contains("dev"));
    #[cfg(all(feature = "web", not(feature = "all")))]
    {
        assert!(help.contains("create"));
        assert!(help.contains("create-web"));
    }
    #[cfg(any(not(feature = "web"), feature = "all"))]
    {
        assert!(help.contains("install-web"));
        assert!(help.contains("create-web"));
        assert!(help.contains("add-web"));
    }

    for core in ["web", "ai", "app", "lib"] {
        assert!(
            help.contains(&format!("create-{core}")),
            "help must expose create-{core}"
        );
        assert!(
            help.contains(&format!("install-{core}")),
            "help must expose install-{core}"
        );
        assert!(
            help.contains(&format!("add-{core}")),
            "help must expose add-{core}"
        );
    }

    #[cfg(any(not(feature = "web"), feature = "all"))]
    assert!(!help.contains("create   "));
}
