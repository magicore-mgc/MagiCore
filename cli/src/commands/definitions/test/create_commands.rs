// Create Commands CLI definition tests.
// Test lược đồ CLI theo nhóm chức năng.
use crate::Cli;
use crate::commands::definitions::{Commands, commands::CreateCommands};
use clap::Parser;

#[test]
fn test_create_web_accepts_flags() {
    let cli = Cli::try_parse_from([
        "mgc",
        "create-web",
        "react@latest",
        "demo-app",
        "--ts",
        "--tailwindcss",
    ])
    .unwrap();

    match cli.command.unwrap() {
        Commands::Create(CreateCommands::CreateWeb {
            framework,
            project_name,
            flags,
        }) => {
            assert_eq!(framework, "react@latest");
            assert_eq!(project_name, "demo-app");
            assert!(flags.ts);
            assert!(flags.tailwindcss);
        }
        _ => panic!("expected create-web command"),
    }
}

#[test]
fn test_create_core_commands_follow_create_core_name_shape() {
    let ai = Cli::try_parse_from(["mgc", "create-ai", "python-agent", "demo-ai"]).unwrap();
    match ai.command.unwrap() {
        Commands::Create(CreateCommands::CreateAi {
            framework,
            project_name,
            container,
            source: _,
            automation: _,
        }) => {
            assert_eq!(framework, "python-agent");
            assert_eq!(project_name, "demo-ai");
            assert!(!container.docker);
            assert!(!container.k8s);
        }
        _ => panic!("expected create-ai command"),
    }

    let app = Cli::try_parse_from(["mgc", "create-app", "flutter", "demo-app"]).unwrap();
    match app.command.unwrap() {
        Commands::Create(CreateCommands::CreateApp {
            framework,
            project_name,
            container,
            source: _,
            automation: _,
        }) => {
            assert_eq!(framework, "flutter");
            assert_eq!(project_name, "demo-app");
            assert!(!container.docker);
            assert!(!container.k8s);
        }
        _ => panic!("expected create-app command"),
    }

    let lib = Cli::try_parse_from(["mgc", "create-lib", "rust", "demo-lib"]).unwrap();
    match lib.command.unwrap() {
        Commands::Create(CreateCommands::CreateLib {
            framework,
            project_name,
            container,
            source: _,
            automation: _,
        }) => {
            assert_eq!(framework, "rust");
            assert_eq!(project_name, "demo-lib");
            assert!(!container.docker);
            assert!(!container.k8s);
        }
        _ => panic!("expected create-lib command"),
    }
}

#[test]
fn test_ai_and_lib_create_commands_accept_container_flags() {
    let ai =
        Cli::try_parse_from(["mgc", "create-ai", "python-agent", "demo-ai", "--docker"]).unwrap();
    match ai.command.unwrap() {
        Commands::Create(CreateCommands::CreateAi { container, .. }) => {
            assert!(container.docker);
            assert!(!container.k8s);
        }
        _ => panic!("expected create-ai command"),
    }

    let lib = Cli::try_parse_from(["mgc", "create-lib", "rust", "demo-lib", "--docker"]).unwrap();
    match lib.command.unwrap() {
        Commands::Create(CreateCommands::CreateLib { container, .. }) => {
            assert!(container.docker);
            assert!(!container.k8s);
        }
        _ => panic!("expected create-lib command"),
    }
}

#[test]
fn test_app_create_command_accepts_docker_flag() {
    let app =
        Cli::try_parse_from(["mgc", "create-app", "flutter", "demo-app", "--docker"]).unwrap();
    match app.command.unwrap() {
        Commands::Create(CreateCommands::CreateApp { container, .. }) => {
            assert!(container.docker);
            assert!(!container.k8s);
        }
        _ => panic!("expected create-app command"),
    }
}

#[test]
fn test_cloud_create_command_accepts_docker_flag() {
    let cloud = Cli::try_parse_from([
        "mgc",
        "create-clo",
        "cdk-typescript",
        "demo-cloud",
        "--docker",
    ])
    .unwrap();
    match cloud.command.unwrap() {
        Commands::Create(CreateCommands::CreateClo { container, .. }) => {
            assert!(container.docker);
            assert!(!container.k8s);
        }
        _ => panic!("expected create-clo command"),
    }
}

#[test]
#[cfg(all(
    feature = "web",
    not(any(
        feature = "game",
        feature = "ai",
        feature = "clo",
        feature = "cicd",
        feature = "iot",
        feature = "app",
        feature = "lib"
    ))
))]
fn test_single_core_create_alias_parses() {
    let cli = Cli::try_parse_from(["mgc", "create", "react@latest", "demo-app", "--ts"]).unwrap();

    match cli.command.unwrap() {
        Commands::Create(CreateCommands::CreateWeb {
            framework,
            project_name,
            flags,
        }) => {
            assert_eq!(framework, "react@latest");
            assert_eq!(project_name, "demo-app");
            assert!(flags.ts);
        }
        _ => panic!("expected create-web command through single-core alias"),
    }
}

/// `--yes` / `--name` must parse on every automation-parity create core.
/// `--yes` / `--name` phải parse được trên mọi core create có cờ tự động hoá.
#[test]
fn test_create_cores_accept_yes_and_name_flags() {
    // Each entry: command, expected non-web variant shape.
    let game =
        Cli::try_parse_from(["mgc", "create-game", "bevy", "--name", "ci-game", "--yes"]).unwrap();
    match game.command.unwrap() {
        Commands::Create(CreateCommands::CreateGame {
            framework,
            project_name,
            automation,
            ..
        }) => {
            assert_eq!(framework, "bevy");
            // Positional PROJECT omitted: defaults to empty, `--name` carries it.
            assert!(project_name.is_empty());
            assert!(automation.yes);
            assert_eq!(automation.name.as_deref(), Some("ci-game"));
        }
        _ => panic!("expected create-game command"),
    }

    let ai = Cli::try_parse_from([
        "mgc",
        "create-ai",
        "python-agent",
        "--name",
        "ci-ai",
        "--yes",
    ])
    .unwrap();
    match ai.command.unwrap() {
        Commands::Create(CreateCommands::CreateAi { automation, .. }) => {
            assert!(automation.yes);
            assert_eq!(automation.name.as_deref(), Some("ci-ai"));
        }
        _ => panic!("expected create-ai command"),
    }

    let cloud =
        Cli::try_parse_from(["mgc", "create-clo", "terraform", "--name", "ci-clo", "-y"]).unwrap();
    match cloud.command.unwrap() {
        Commands::Create(CreateCommands::CreateClo { automation, .. }) => {
            assert!(automation.yes);
            assert_eq!(automation.name.as_deref(), Some("ci-clo"));
        }
        _ => panic!("expected create-clo command"),
    }

    let cicd = Cli::try_parse_from([
        "mgc",
        "create-cicd",
        "github-actions",
        "--name",
        "ci-cd",
        "--yes",
    ])
    .unwrap();
    match cicd.command.unwrap() {
        Commands::Create(CreateCommands::CreateCicd { automation, .. }) => {
            assert!(automation.yes);
            assert_eq!(automation.name.as_deref(), Some("ci-cd"));
        }
        _ => panic!("expected create-cicd command"),
    }

    let iot = Cli::try_parse_from([
        "mgc",
        "create-iot",
        "esp32-rust",
        "--name",
        "ci-iot",
        "--yes",
    ])
    .unwrap();
    match iot.command.unwrap() {
        Commands::Create(CreateCommands::CreateIot { automation, .. }) => {
            assert!(automation.yes);
            assert_eq!(automation.name.as_deref(), Some("ci-iot"));
        }
        _ => panic!("expected create-iot command"),
    }

    let app =
        Cli::try_parse_from(["mgc", "create-app", "flutter", "--name", "ci-app", "--yes"]).unwrap();
    match app.command.unwrap() {
        Commands::Create(CreateCommands::CreateApp { automation, .. }) => {
            assert!(automation.yes);
            assert_eq!(automation.name.as_deref(), Some("ci-app"));
        }
        _ => panic!("expected create-app command"),
    }

    let lib =
        Cli::try_parse_from(["mgc", "create-lib", "rust", "--name", "ci-lib", "--yes"]).unwrap();
    match lib.command.unwrap() {
        Commands::Create(CreateCommands::CreateLib { automation, .. }) => {
            assert!(automation.yes);
            assert_eq!(automation.name.as_deref(), Some("ci-lib"));
        }
        _ => panic!("expected create-lib command"),
    }

    let hardware = Cli::try_parse_from([
        "mgc",
        "create-hardware",
        "bench",
        "--name",
        "ci-hw",
        "--yes",
    ])
    .unwrap();
    match hardware.command.unwrap() {
        Commands::Create(CreateCommands::CreateHardware { automation, .. }) => {
            assert!(automation.yes);
            assert_eq!(automation.name.as_deref(), Some("ci-hw"));
        }
        _ => panic!("expected create-hardware command"),
    }
}

/// Without `--yes`/`--name` the legacy positional shape still parses unchanged.
/// Không có `--yes`/`--name` thì hình dạng vị trí cũ vẫn parse y nguyên.
#[test]
fn test_create_cores_keep_positional_shape_without_automation_flags() {
    let game = Cli::try_parse_from(["mgc", "create-game", "bevy", "legacy-game"]).unwrap();
    match game.command.unwrap() {
        Commands::Create(CreateCommands::CreateGame {
            framework,
            project_name,
            automation,
            ..
        }) => {
            assert_eq!(framework, "bevy");
            assert_eq!(project_name, "legacy-game");
            assert!(!automation.yes);
            assert!(automation.name.is_none());
        }
        _ => panic!("expected create-game command"),
    }
}

/// `--help` for each automated core must advertise `--yes` and `--name`.
/// `--help` của mỗi core tự động hoá phải hiển thị `--yes` và `--name`.
#[test]
fn test_create_core_help_lists_automation_flags() {
    use clap::CommandFactory;

    for core in [
        "create-game",
        "create-ai",
        "create-clo",
        "create-cicd",
        "create-iot",
        "create-app",
        "create-lib",
        "create-hardware",
    ] {
        let mut command = Cli::command();
        let sub = command
            .find_subcommand_mut(core)
            .unwrap_or_else(|| panic!("{core} subcommand must exist"));
        let help = sub.render_long_help().to_string();
        assert!(help.contains("--yes"), "{core} --help must list --yes");
        assert!(help.contains("--name"), "{core} --help must list --name");
    }
}
