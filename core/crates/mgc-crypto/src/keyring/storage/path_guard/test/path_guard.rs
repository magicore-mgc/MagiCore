//! Keyring path policy tests — Test policy path keyring.
//! These checks exercise the production path validator — Kiểm tra validator production.

use super::path_guard::{validate_keyring_path, validate_keyring_path_under};

#[test]
fn accepts_a_destination_under_canonical_home() {
    let home = dirs::home_dir().expect("home directory is required for keyring path tests");
    let home = home.canonicalize().unwrap_or(home);
    let destination = home.join(".magicore").join("keys").join("candidate.json");
    assert!(validate_keyring_path(&destination).is_ok());
}

#[test]
fn rejects_a_destination_outside_home() {
    let home = dirs::home_dir().expect("home directory is required for keyring path tests");
    let parent = home.parent().unwrap_or(home.as_path());
    let outside = parent.join("mgc-keyring-outside-candidate.json");
    assert!(validate_keyring_path(&outside).is_err());
}

#[test]
#[cfg(unix)]
fn resolves_a_symlinked_home_before_checking_a_missing_destination() {
    use std::os::unix::fs::symlink;

    let directory = tempfile::tempdir().expect("test operation must succeed");
    let home = directory.path().join("real-home");
    let alias = directory.path().join("home-alias");
    std::fs::create_dir(&home).expect("test operation must succeed");
    symlink(&home, &alias).expect("test operation must succeed");
    let destination = alias.join(".magicore").join("keys").join("keyring.json");

    assert!(validate_keyring_path_under(&destination, &alias).is_ok());
}

#[test]
fn rejects_parent_traversal_after_a_missing_path_component() {
    let directory = tempfile::tempdir().expect("test operation must succeed");
    let home = directory.path().join("home");
    std::fs::create_dir(&home).expect("test operation must succeed");
    let destination = home.join("not-created").join("..");
    let destination = destination.join("..").join("outside").join("keyring.json");

    assert!(validate_keyring_path_under(&destination, &home).is_err());
}
