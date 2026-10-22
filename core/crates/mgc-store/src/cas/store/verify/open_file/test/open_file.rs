//! Tests stable CAS diagnostics for invalid verification paths.
//! Kiểm thử diagnostic CAS ổn định cho path xác minh không hợp lệ.

use super::super::open_regular_file;
use crate::cas::store::StoreError;

#[test]
fn rejects_a_directory_with_the_existing_diagnostic() {
    let temp = tempfile::tempdir().unwrap();
    let directory = temp.path().join("directory");
    std::fs::create_dir(&directory).unwrap();

    let error = open_regular_file(&directory, "path is a symlink, refusing to verify").unwrap_err();

    assert!(matches!(error, StoreError::Io { msg, .. } if msg == "path is not a regular file"));
}

#[cfg(unix)]
#[test]
fn rejects_a_symlink_with_the_existing_diagnostic() {
    let temp = tempfile::tempdir().unwrap();
    let target = temp.path().join("target");
    let link = temp.path().join("link");
    std::fs::write(&target, b"target").unwrap();
    std::os::unix::fs::symlink(&target, &link).unwrap();

    let error = open_regular_file(&link, "path is a symlink, refusing to fingerprint").unwrap_err();

    assert!(
        matches!(error, StoreError::Io { msg, .. } if msg == "path is a symlink, refusing to fingerprint")
    );
}
