#![allow(clippy::unwrap_used)]

use super::super::*;

#[test]
fn rejects_empty_or_whitespace_tokens_without_creating_a_file() {
    let directory = tempfile::tempdir().unwrap();
    let path = directory.path().join(".npmrc");
    assert!(NpmRc::save_auth_token(&path, "registry.example", "").is_err());
    assert!(NpmRc::save_auth_token(&path, "registry.example", "bad token").is_err());
    assert!(!path.exists());
}

#[cfg(unix)]
#[test]
fn refuses_symlink_target_and_writes_new_file_with_private_mode() {
    use std::os::unix::fs::{PermissionsExt, symlink};

    let directory = tempfile::tempdir().unwrap();
    let target = directory.path().join("target");
    std::fs::write(&target, "registry=https://keep.example/\n").unwrap();
    let link = directory.path().join(".npmrc-link");
    symlink(&target, &link).unwrap();
    assert!(NpmRc::save_auth_token(&link, "registry.example", "fixture-token").is_err());
    assert_eq!(
        std::fs::read_to_string(&target).unwrap(),
        "registry=https://keep.example/\n"
    );

    let path = directory.path().join(".npmrc");
    NpmRc::save_auth_token_for_registry_url(
        &path,
        "https://registry.example/team",
        "fixture-token",
    )
    .unwrap();
    let mode = std::fs::metadata(&path).unwrap().permissions().mode() & 0o777;
    assert_eq!(mode, 0o600);
}
