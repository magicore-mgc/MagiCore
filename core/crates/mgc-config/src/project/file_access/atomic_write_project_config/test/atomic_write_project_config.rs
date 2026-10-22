//! Tests for same-directory atomic project-config replacement.
//! Test thay thế config project atomic cùng thư mục.
use super::super::atomic_write_project_config;

#[test]
fn creates_and_replaces_config_contents() {
    let project = tempfile::tempdir().expect("temporary project should be created");
    let path = project.path().join("mgc.toml");

    atomic_write_project_config(&path, b"ecosystem = 'web'\n").unwrap();
    assert_eq!(std::fs::read(&path).unwrap(), b"ecosystem = 'web'\n");
    atomic_write_project_config(&path, b"ecosystem = 'ai'\n").unwrap();
    assert_eq!(std::fs::read(&path).unwrap(), b"ecosystem = 'ai'\n");
}

#[cfg(unix)]
#[test]
fn existing_config_permissions_are_preserved() {
    use std::os::unix::fs::{MetadataExt, PermissionsExt};

    let project = tempfile::tempdir().expect("temporary project should be created");
    let path = project.path().join("mgc.toml");
    std::fs::write(&path, b"ecosystem = 'web'\n").expect("config should be written");
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o640))
        .expect("permissions should be set");
    let original_mode = std::fs::metadata(&path).unwrap().mode() & 0o777;

    atomic_write_project_config(&path, b"ecosystem = 'ai'\n").unwrap();

    assert_eq!(
        std::fs::metadata(&path).unwrap().mode() & 0o777,
        original_mode
    );
}

#[cfg(unix)]
#[test]
fn symlink_destination_is_rejected_and_target_is_unchanged() {
    use std::os::unix::fs::symlink;

    let project = tempfile::tempdir().expect("temporary project should be created");
    let target = project.path().join("target.toml");
    let link = project.path().join("mgc.toml");
    std::fs::write(&target, b"ecosystem = 'web'\n").expect("target should be written");
    symlink(&target, &link).expect("symlink should be created");

    let error = atomic_write_project_config(&link, b"ecosystem = 'ai'\n")
        .expect_err("symlink destination must be rejected");

    assert!(error.to_string().contains("must not be a symlink"));
    assert_eq!(std::fs::read(&target).unwrap(), b"ecosystem = 'web'\n");
}

#[cfg(windows)]
#[test]
fn reparse_destination_is_rejected_and_target_is_unchanged() {
    use std::os::windows::fs::symlink_file;

    let project = tempfile::tempdir().expect("temporary project should be created");
    let target = project.path().join("target.toml");
    let link = project.path().join("mgc.toml");
    std::fs::write(&target, b"ecosystem = 'web'\n").expect("target should be written");
    symlink_file(&target, &link).expect("file symlink should be created");

    let error = atomic_write_project_config(&link, b"ecosystem = 'ai'\n")
        .expect_err("reparse destination must be rejected");

    assert!(error.to_string().contains("must not be a symlink"));
    assert_eq!(std::fs::read(&target).unwrap(), b"ecosystem = 'web'\n");
}
