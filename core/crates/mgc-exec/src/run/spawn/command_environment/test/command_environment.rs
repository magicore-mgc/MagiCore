//! Verify guarded environment assignment without launching a child.
//! Kiểm tra gán environment có bảo vệ mà không launch child.

use super::super::configure_command_environment;
use crate::run::implementation::ExecOptions;
use std::ffi::OsStr;
use std::process::Command;

#[cfg(test)]
fn assert_explicit_path_is_guarded(clean_env: bool) {
    let root = tempfile::tempdir().expect("create command environment test root");
    let shadow_dir = root.path().join("deny shim directory");
    let caller_bin = root.path().join("caller bin");
    std::fs::create_dir(&shadow_dir).expect("create shadow path");
    std::fs::create_dir(&caller_bin).expect("create caller PATH entry");
    let options = ExecOptions {
        clean_env,
        env: vec![
            ("PATH".to_owned(), caller_bin.to_string_lossy().into_owned()),
            ("MGC_TEST_ENV".to_owned(), "retained".to_owned()),
        ],
        ..ExecOptions::default()
    };
    let mut command = Command::new("placeholder-command");

    configure_command_environment(&mut command, &options, &shadow_dir)
        .expect("configure command environment");

    let applied_path = command
        .get_envs()
        .find(|(key, _)| crate::run::implementation::is_path_env_key(&key.to_string_lossy()))
        .and_then(|(_, value)| value)
        .expect("guarded PATH is explicitly configured");
    let path_entries = std::env::split_paths(applied_path).collect::<Vec<_>>();
    assert_eq!(path_entries.first(), Some(&shadow_dir));
    assert!(path_entries.contains(&caller_bin));

    let retained = command
        .get_envs()
        .find(|(key, _)| *key == OsStr::new("MGC_TEST_ENV"))
        .and_then(|(_, value)| value)
        .expect("explicit non-PATH environment remains configured");
    assert_eq!(retained, OsStr::new("retained"));
}

#[test]
fn inherited_environment_keeps_deny_shims_before_explicit_path() {
    assert_explicit_path_is_guarded(false);
}

#[test]
fn clean_environment_keeps_deny_shims_and_explicit_values() {
    assert_explicit_path_is_guarded(true);
}
