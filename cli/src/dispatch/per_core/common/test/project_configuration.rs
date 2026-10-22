// Project configuration mapping — retain nested config and migration values.
// Mapping cấu hình dự án — giữ giá trị config và migration lồng nhau.
use super::super::common_to_dispatch;
use crate::commands::config::ConfigCmd;
use crate::commands::definitions::commands::common::CommonCommands;
use crate::commands::migrate::MigrateCmd;
use crate::dispatch::types::CommonCommand;
use std::path::PathBuf;

fn mapped(command: CommonCommands) -> CommonCommand {
    match common_to_dispatch(command) {
        Some(command) => command,
        None => panic!("common command must be mapped"),
    }
}

#[test]
fn maps_configuration_scope_and_project_paths() {
    match mapped(CommonCommands::Config {
        cmd: ConfigCmd::Set {
            key: "registry".to_owned(),
            value: "https://registry.example".to_owned(),
            toml: true,
        },
        local: true,
    }) {
        CommonCommand::Config { cmd, local } => {
            assert!(local);
            match cmd {
                ConfigCmd::Set { key, value, toml } => {
                    assert_eq!(key, "registry");
                    assert_eq!(value, "https://registry.example");
                    assert!(toml);
                }
                _ => panic!("config subcommand was changed"),
            }
        }
        _ => panic!("config command mapped to a different variant"),
    }

    let stage_dir = PathBuf::from("stage-project");
    match mapped(CommonCommands::Stage {
        dir: Some(stage_dir.clone()),
    }) {
        CommonCommand::Stage { dir } => assert_eq!(dir, Some(stage_dir)),
        _ => panic!("stage command mapped to a different variant"),
    }

    let import_dir = PathBuf::from("legacy-project");
    match mapped(CommonCommands::Import {
        dir: Some(import_dir.clone()),
        allow_unsigned: true,
    }) {
        CommonCommand::Import {
            dir,
            allow_unsigned,
        } => {
            assert_eq!(dir, Some(import_dir));
            assert!(allow_unsigned);
        }
        _ => panic!("import command mapped to a different variant"),
    }
}

#[test]
fn maps_lockfile_migration_target_and_directory() {
    let migration_dir = PathBuf::from("migration-project");
    match mapped(CommonCommands::Migrate {
        cmd: MigrateCmd::Lock {
            to: "v4".to_owned(),
            dir: Some(migration_dir.clone()),
        },
    }) {
        CommonCommand::Migrate {
            cmd: MigrateCmd::Lock { to, dir },
        } => {
            assert_eq!(to, "v4");
            assert_eq!(dir, Some(migration_dir));
        }
        _ => panic!("migration command or payload was changed"),
    }
}
