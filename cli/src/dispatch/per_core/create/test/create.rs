// Create adapter tests — cover all core variants and their distinct payloads.
// Test adapter create — bao phủ variant và payload riêng của từng core.
use super::*;
use crate::commands::core::shared::container_flags::ContainerFlags;
use crate::commands::core::web::scaffold_flags::WebScaffoldFlags;

fn mapped(command: CreateCommands) -> CoreCommand {
    match map(command) {
        Some(command) => command,
        None => panic!("create command must be handled"),
    }
}

macro_rules! assert_basic_variant {
    ($input:expr, $variant:ident) => {{
        match mapped($input) {
            CoreCommand::$variant {
                framework,
                project_name,
            } => {
                assert_eq!(framework, "sample-framework");
                assert_eq!(project_name, "sample-project");
            }
            _ => panic!("create adapter produced a different basic variant"),
        }
    }};
}

macro_rules! assert_container_variant {
    ($input:expr, $variant:ident, $docker:expr, $k8s:expr) => {{
        match mapped($input) {
            CoreCommand::$variant {
                framework,
                project_name,
                container,
            } => {
                assert_eq!(framework, "sample-framework");
                assert_eq!(project_name, "sample-project");
                assert_eq!(container.docker, $docker);
                assert_eq!(container.k8s, $k8s);
            }
            _ => panic!("create adapter produced a different container variant"),
        }
    }};
}

#[test]
fn maps_all_basic_framework_project_variants() {
    assert_basic_variant!(
        CreateCommands::CreateGame {
            framework: "sample-framework".to_owned(),
            project_name: "sample-project".to_owned(),
        },
        CreateGame
    );
    assert_basic_variant!(
        CreateCommands::CreateCicd {
            framework: "sample-framework".to_owned(),
            project_name: "sample-project".to_owned(),
        },
        CreateCicd
    );
    assert_basic_variant!(
        CreateCommands::CreateIot {
            framework: "sample-framework".to_owned(),
            project_name: "sample-project".to_owned(),
        },
        CreateIot
    );
    assert_basic_variant!(
        CreateCommands::CreateHardware {
            framework: "sample-framework".to_owned(),
            project_name: "sample-project".to_owned(),
        },
        CreateHardware
    );
}

#[test]
fn maps_all_container_payloads_without_losing_flags() {
    assert_container_variant!(
        CreateCommands::CreateAi {
            framework: "sample-framework".to_owned(),
            project_name: "sample-project".to_owned(),
            container: ContainerFlags {
                docker: true,
                k8s: false
            },
        },
        CreateAi,
        true,
        false
    );
    assert_container_variant!(
        CreateCommands::CreateClo {
            framework: "sample-framework".to_owned(),
            project_name: "sample-project".to_owned(),
            container: ContainerFlags {
                docker: false,
                k8s: true
            },
        },
        CreateClo,
        false,
        true
    );
    assert_container_variant!(
        CreateCommands::CreateApp {
            framework: "sample-framework".to_owned(),
            project_name: "sample-project".to_owned(),
            container: ContainerFlags {
                docker: true,
                k8s: true
            },
        },
        CreateApp,
        true,
        true
    );
    assert_container_variant!(
        CreateCommands::CreateLib {
            framework: "sample-framework".to_owned(),
            project_name: "sample-project".to_owned(),
            container: ContainerFlags {
                docker: false,
                k8s: false
            },
        },
        CreateLib,
        false,
        false
    );
}

#[test]
fn maps_web_project_identity_and_scaffold_flags() {
    let flags = WebScaffoldFlags {
        name: Some("display-name".to_owned()),
        install: false,
        git: true,
        docker: true,
        k8s: true,
        preset: Some("starter".to_owned()),
        ..WebScaffoldFlags::default()
    };

    match mapped(CreateCommands::CreateWeb {
        framework: "sample-framework".to_owned(),
        project_name: "sample-project".to_owned(),
        flags,
    }) {
        CoreCommand::CreateWeb {
            framework,
            project_name,
            flags,
        } => {
            assert_eq!(framework, "sample-framework");
            assert_eq!(project_name, "sample-project");
            assert_eq!(flags.name.as_deref(), Some("display-name"));
            assert!(!flags.install);
            assert!(flags.git);
            assert!(flags.docker);
            assert!(flags.k8s);
            assert_eq!(flags.preset.as_deref(), Some("starter"));
        }
        _ => panic!("create adapter produced a different Web variant"),
    }
}
