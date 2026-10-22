// Package Lifecycle CLI definition tests.
// Test lược đồ CLI theo nhóm chức năng.
use crate::Cli;
use crate::commands::definitions::Commands;
use crate::commands::definitions::commands::{
    AddCommands, BareCommands, CommonCommands, InstallCommands, RemoveCommands, UpdateCommands,
};
use clap::Parser;

#[test]
fn test_add_web_accepts_multiple_packages() {
    let cli =
        Cli::try_parse_from(["mgc", "add-web", "zod", "lodash", "@types/node", "-D"]).unwrap();

    match cli.command.unwrap() {
        Commands::Add(AddCommands::AddWeb { packages, dev, .. }) => {
            assert_eq!(packages, vec!["zod", "lodash", "@types/node"]);
            assert!(dev);
        }
        _ => panic!("expected add-web command"),
    }
}

#[test]
fn test_add_and_remove_accept_no_install() {
    let add = Cli::try_parse_from(["mgc", "add-web", "dayjs", "--no-install"]).unwrap();
    match add.command.unwrap() {
        Commands::Add(AddCommands::AddWeb { no_install, .. }) => assert!(no_install),
        _ => panic!("expected add-web command"),
    }

    let remove =
        Cli::try_parse_from(["mgc", "remove-web", "zod", "lodash", "--no-install"]).unwrap();
    match remove.command.unwrap() {
        Commands::Remove(RemoveCommands::RemoveWeb {
            packages,
            no_install,
            ..
        }) => {
            assert_eq!(packages, vec!["zod", "lodash"]);
            assert!(no_install);
        }
        _ => panic!("expected remove-web command"),
    }
}

#[test]
fn test_add_accepts_version_pin() {
    // T0.3-version: bare `add --version` must parse (the pin flows to
    // lanes that honor it instead of being dropped).
    let add = Cli::try_parse_from(["mgc", "add", "zod", "--version", "3.22.4"]).unwrap();
    match add.command.unwrap() {
        Commands::Bare(BareCommands::Add {
            packages, version, ..
        }) => {
            assert_eq!(packages, vec!["zod"]);
            assert_eq!(version.as_deref(), Some("3.22.4"));
        }
        _ => panic!("expected add command"),
    }
}

#[test]
fn test_compat_runtime_flag_parses_on_dependency_commands() {
    // C0 firewall UX (T0.3): --compat-runtime must parse on every
    // install/add/remove/update form (bare + per-core).
    let install = Cli::try_parse_from(["mgc", "install", "--compat-runtime", "uv"]).unwrap();
    match install.command.unwrap() {
        Commands::Bare(BareCommands::Install { compat_runtime, .. }) => {
            assert_eq!(compat_runtime.as_deref(), Some("uv"))
        }
        _ => panic!("expected install command"),
    }

    let install_ai = Cli::try_parse_from(["mgc", "install-ai", "--compat-runtime", "uv"]).unwrap();
    match install_ai.command.unwrap() {
        Commands::Install(InstallCommands::InstallAi { compat_runtime, .. }) => {
            assert_eq!(compat_runtime.as_deref(), Some("uv"))
        }
        _ => panic!("expected install-ai command"),
    }

    let add_app =
        Cli::try_parse_from(["mgc", "add-app", "pkg", "--compat-runtime", "flutter"]).unwrap();
    match add_app.command.unwrap() {
        Commands::Add(AddCommands::AddApp { compat_runtime, .. }) => {
            assert_eq!(compat_runtime.as_deref(), Some("flutter"))
        }
        _ => panic!("expected add-app command"),
    }

    let remove = Cli::try_parse_from(["mgc", "remove", "zod"]).unwrap();
    match remove.command.unwrap() {
        Commands::Bare(BareCommands::Remove { compat_runtime, .. }) => {
            assert!(compat_runtime.is_none())
        }
        _ => panic!("expected remove command"),
    }

    let update_lib =
        Cli::try_parse_from(["mgc", "update-lib", "--compat-runtime", "cargo"]).unwrap();
    match update_lib.command.unwrap() {
        Commands::Update(UpdateCommands::UpdateLib { compat_runtime, .. }) => {
            assert_eq!(compat_runtime.as_deref(), Some("cargo"))
        }
        _ => panic!("expected update-lib command"),
    }
}

#[test]
fn test_install_accepts_script_policy_flags() {
    let install = Cli::try_parse_from(["mgc", "install", "--allow-scripts"]).unwrap();
    match install.command.unwrap() {
        Commands::Bare(BareCommands::Install {
            ignore_scripts,
            allow_scripts,
            ..
        }) => {
            assert!(!ignore_scripts);
            assert!(allow_scripts);
        }
        _ => panic!("expected install command"),
    }

    let install_web =
        Cli::try_parse_from(["mgc", "install-web", "--ignore-scripts", "--allow-scripts"]).unwrap();
    match install_web.command.unwrap() {
        Commands::Install(InstallCommands::InstallWeb {
            ignore_scripts,
            allow_scripts,
            ..
        }) => {
            assert!(ignore_scripts);
            assert!(allow_scripts);
        }
        _ => panic!("expected install-web command"),
    }
}

#[test]
fn test_publish_lifecycle_requires_explicit_opt_in_flag() {
    let defaults = Cli::try_parse_from(["mgc", "publish", "--dry-run"]).unwrap();
    match defaults.command.unwrap() {
        Commands::Common(CommonCommands::Publish {
            allow_scripts,
            ignore_scripts,
            ..
        }) => {
            assert!(!allow_scripts);
            assert!(!ignore_scripts);
        }
        _ => panic!("expected publish command"),
    }

    let allowed = Cli::try_parse_from(["mgc", "publish", "--allow-scripts", "--dry-run"]).unwrap();
    match allowed.command.unwrap() {
        Commands::Common(CommonCommands::Publish { allow_scripts, .. }) => assert!(allow_scripts),
        _ => panic!("expected publish command with explicit script approval"),
    }

    assert!(
        Cli::try_parse_from(["mgc", "publish", "--allow-scripts", "--ignore-scripts",]).is_err()
    );
}

#[test]
fn native_game_cloud_and_iot_install_commands_parse_frozen() {
    for command in [
        ["mgc", "install-game", "--frozen"],
        ["mgc", "install-clo", "--frozen"],
        ["mgc", "install-iot", "--frozen"],
    ] {
        let parsed = Cli::try_parse_from(command).unwrap();
        match parsed.command.unwrap() {
            Commands::Install(InstallCommands::InstallGame { frozen, .. })
            | Commands::Install(InstallCommands::InstallClo { frozen, .. })
            | Commands::Install(InstallCommands::InstallIot { frozen, .. }) => assert!(frozen),
            _ => panic!("expected a frozen native install command"),
        }
    }
}

#[test]
fn test_install_accepts_package_specs() {
    let install = Cli::try_parse_from([
        "mgc",
        "install",
        "react@latest",
        "zod@^3.22.4",
        "--allow-scripts",
    ])
    .unwrap();

    match install.command.unwrap() {
        Commands::Bare(BareCommands::Install {
            packages,
            allow_scripts,
            ..
        }) => {
            assert_eq!(packages, vec!["react@latest", "zod@^3.22.4"]);
            assert!(allow_scripts);
        }
        _ => panic!("expected install command"),
    }
}

#[test]
fn test_install_parses_dry_run_flag() {
    let cli = Cli::try_parse_from(["mgc", "install", "--dry-run"]).unwrap();
    match cli.command.unwrap() {
        Commands::Bare(BareCommands::Install { dry_run, .. }) => assert!(dry_run),
        _ => panic!("expected install command"),
    }
}
