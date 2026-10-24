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
