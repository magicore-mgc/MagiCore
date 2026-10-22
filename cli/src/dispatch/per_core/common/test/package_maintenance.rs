// Package maintenance mapping — retain nested patch and account options.
// Mapping quản lý package — giữ tùy chọn patch lồng nhau và tài khoản.
use super::super::common_to_dispatch;
use crate::commands::definitions::commands::common::CommonCommands;
use crate::commands::patch::PatchCmd;
use crate::dispatch::types::CommonCommand;

#[test]
fn maps_patch_and_dedupe_options() {
    match common_to_dispatch(CommonCommands::Patch {
        cmd: PatchCmd::Add {
            package: "react".to_owned(),
            file: "patches/react.patch".to_owned(),
            range: Some("1.0.0 - 2.0.0".to_owned()),
        },
    }) {
        Some(CommonCommand::Patch {
            cmd:
                PatchCmd::Add {
                    package,
                    file,
                    range,
                },
        }) => {
            assert_eq!(package, "react");
            assert_eq!(file, "patches/react.patch");
            assert_eq!(range.as_deref(), Some("1.0.0 - 2.0.0"));
        }
        _ => panic!("patch command or payload was changed"),
    }

    match common_to_dispatch(CommonCommands::Dedupe {
        dry_run: true,
        prefer_latest: true,
        json: true,
    }) {
        Some(CommonCommand::Dedupe {
            dry_run,
            prefer_latest,
            json,
        }) => {
            assert!(dry_run);
            assert!(prefer_latest);
            assert!(json);
        }
        _ => panic!("dedupe command or payload was changed"),
    }
}

#[test]
fn maps_login_registry_credentials_and_scope_options() {
    match common_to_dispatch(CommonCommands::Login {
        registry: Some("https://registry.example".to_owned()),
        username: Some("publisher".to_owned()),
        password_stdin: true,
        local: true,
    }) {
        Some(CommonCommand::Login {
            registry,
            username,
            password_stdin,
            local,
        }) => {
            assert_eq!(registry.as_deref(), Some("https://registry.example"));
            assert_eq!(username.as_deref(), Some("publisher"));
            assert!(password_stdin);
            assert!(local);
        }
        _ => panic!("login command or payload was changed"),
    }
}
