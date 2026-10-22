#![cfg(test)]
#![allow(clippy::unwrap_used)]
//! Dispatch behavior tests that do not belong to a narrower engine owner.
//! Test hành vi dispatch chưa thuộc owner engine hẹp hơn.

use super::*;
use crate::commands::definitions::commands::{AddCommands, BareCommands};

#[test]
fn bare_add_version_pin_reaches_core_command() {
    use crate::dispatch::bare::bare_core_command;
    use crate::dispatch::types::DispatchCommand;
    let command = BareCommands::Add {
        packages: vec!["zod".into()],
        version: Some("3.22.4".into()),
        dev: false,
        global: false,
        exact: false,
        optional: false,
        peer: false,
        no_save: false,
        no_install: false,
        compat_runtime: None,
    };
    match bare_core_command(command, Some("ai".to_string())).unwrap() {
        DispatchCommand::Core(crate::dispatch::types::CoreCommand::AddAi { version, .. }) => {
            assert_eq!(version.as_deref(), Some("3.22.4"))
        }
        _ => panic!("expected AddAi core command"),
    }
}

#[test]
fn bare_install_frozen_reaches_game_cloud_and_iot_lanes() {
    use crate::dispatch::bare::bare_core_command;
    use crate::dispatch::types::{CoreCommand, DispatchCommand};

    for core in ["game", "clo", "iot"] {
        let command = BareCommands::Install {
            packages: Vec::new(),
            frozen: true,
            ignore_scripts: false,
            allow_scripts: false,
            prefer_dedupe: false,
            repair: false,
            dry_run: false,
            offline: false,
            compat_runtime: None,
        };
        let dispatch = bare_core_command(command, Some(core.to_string())).unwrap();
        match (core, dispatch) {
            ("game", DispatchCommand::Core(CoreCommand::InstallGame { frozen, .. }))
            | ("clo", DispatchCommand::Core(CoreCommand::InstallClo { frozen, .. }))
            | ("iot", DispatchCommand::Core(CoreCommand::InstallIot { frozen, .. })) => {
                assert!(frozen, "mgc install --core {core} must preserve --frozen")
            }
            (_, _) => panic!("unexpected install dispatch for core {core}"),
        }
    }
}

#[test]
fn per_core_add_version_pin_reaches_lane() {
    // T0.3-version parity: per-core `add-* --version` forwards the pin
    // (previously only bare `add` even parsed it, then dropped it).
    use crate::dispatch::per_core::command_to_dispatch;
    use crate::dispatch::types::DispatchCommand;
    let command = Commands::Add(AddCommands::AddLib {
        packages: vec!["serde".into()],
        dev: false,
        exact: false,
        optional: false,
        peer: false,
        no_save: false,
        global: false,
        compat_runtime: None,
        version: Some("1.0.0".into()),
    });
    match command_to_dispatch(command, None).unwrap() {
        DispatchCommand::Core(crate::dispatch::types::CoreCommand::AddLib { version, .. }) => {
            assert_eq!(version.as_deref(), Some("1.0.0"))
        }
        _ => panic!("expected AddLib core command"),
    }
}
