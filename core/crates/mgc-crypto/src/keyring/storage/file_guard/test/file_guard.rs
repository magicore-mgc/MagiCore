//! Existing keyring path tests — Test path keyring hiện có.
//! Exercise backup reads and replacement guards — Kiểm tra đọc backup và guard replace.

use super::file_guard::{read_existing_for_backup, reject_non_regular_existing_path};
use std::fs;

#[test]
fn allows_missing_destination_for_first_save() {
    let directory = tempfile::tempdir().expect("test operation must succeed");
    let missing = directory.path().join("keyring.json");
    assert!(reject_non_regular_existing_path(&missing).is_ok());
}

#[test]
fn refuses_directory_as_replacement_destination() {
    let directory = tempfile::tempdir().expect("test operation must succeed");
    assert!(reject_non_regular_existing_path(directory.path()).is_err());
}

#[test]
fn reads_existing_file_contents_for_backup() {
    let directory = tempfile::tempdir().expect("test operation must succeed");
    let path = directory.path().join("keyring.json");
    fs::write(&path, b"backup contents").expect("test operation must succeed");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&path, fs::Permissions::from_mode(0o600))
            .expect("test operation must succeed");
    }
    assert_eq!(
        read_existing_for_backup(&path)
            .expect("test operation must succeed")
            .as_deref(),
        Some("backup contents")
    );
}

#[test]
#[cfg(unix)]
fn refuses_symlink_replacement_destination() {
    use std::os::unix::fs::symlink;

    let directory = tempfile::tempdir().expect("test operation must succeed");
    let target = directory.path().join("outside.json");
    let link = directory.path().join("keyring.json");
    fs::write(&target, b"unchanged").expect("test operation must succeed");
    symlink(&target, &link).expect("test operation must succeed");
    assert!(reject_non_regular_existing_path(&link).is_err());
}
