use super::super::{PackageCore, owner_for};
use crate::dispatch::types::CoreCommand;

#[test]
fn selects_exactly_the_matching_core_owner() {
    let cases = [
        (
            CoreCommand::ListWeb {
                compat_runtime: None,
            },
            PackageCore::Web,
        ),
        (
            CoreCommand::ListGame {
                compat_runtime: None,
            },
            PackageCore::Game,
        ),
        (
            CoreCommand::ListAi {
                compat_runtime: None,
            },
            PackageCore::Ai,
        ),
        (
            CoreCommand::ListClo {
                compat_runtime: None,
            },
            PackageCore::Clo,
        ),
        (
            CoreCommand::ListCicd {
                compat_runtime: None,
            },
            PackageCore::Cicd,
        ),
        (
            CoreCommand::ListIot {
                compat_runtime: None,
            },
            PackageCore::Iot,
        ),
        (
            CoreCommand::ListApp {
                compat_runtime: None,
            },
            PackageCore::App,
        ),
        (
            CoreCommand::ListLib {
                compat_runtime: None,
            },
            PackageCore::Lib,
        ),
        (
            CoreCommand::ListHardware {
                compat_runtime: None,
            },
            PackageCore::Hardware,
        ),
    ];

    for (command, expected) in cases {
        assert_eq!(owner_for(&command), Some(expected));
    }
}

#[test]
fn rejects_create_and_install_commands() {
    let create = CoreCommand::CreateGame {
        framework: "fixture".to_owned(),
        project_name: "fixture".to_owned(),
        source: Default::default(),
        automation: Default::default(),
    };
    let install = CoreCommand::InstallWeb {
        packages: Vec::new(),
        frozen: false,
        ignore_scripts: false,
        allow_scripts: false,
        prefer_dedupe: false,
        repair: false,
        offline: false,
        compat_runtime: None,
    };

    assert_eq!(owner_for(&create), None);
    assert_eq!(owner_for(&install), None);
}
