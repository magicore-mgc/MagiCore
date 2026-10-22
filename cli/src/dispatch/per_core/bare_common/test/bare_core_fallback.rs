// Core package operations — prove they remain outside shared dispatch.
// Package operation theo core — xác minh chúng không vào dispatch chung.
use super::super::bare_common_to_common;
use crate::commands::definitions::commands::bare::BareCommands;

#[test]
fn install_add_remove_update_and_list_fall_through_to_core_routing() {
    let package_operation_commands = [
        BareCommands::Install {
            packages: vec!["react".to_owned()],
            frozen: true,
            ignore_scripts: true,
            allow_scripts: false,
            prefer_dedupe: true,
            repair: true,
            dry_run: true,
            offline: true,
            compat_runtime: Some("bun".to_owned()),
        },
        BareCommands::Add {
            packages: vec!["react".to_owned()],
            version: Some("18.3.1".to_owned()),
            dev: true,
            global: false,
            exact: true,
            optional: true,
            peer: false,
            no_save: false,
            no_install: true,
            compat_runtime: Some("pnpm".to_owned()),
        },
        BareCommands::Remove {
            packages: vec!["react".to_owned()],
            no_install: true,
            compat_runtime: Some("npm".to_owned()),
        },
        BareCommands::Update {
            packages: vec!["react".to_owned()],
            install: true,
            compat_runtime: Some("yarn".to_owned()),
        },
        BareCommands::List {
            compat_runtime: Some("bun".to_owned()),
        },
    ];

    for command in package_operation_commands {
        assert!(bare_common_to_common(command).is_none());
    }
}
