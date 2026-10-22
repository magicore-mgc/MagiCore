//! Archive path rejection tests.
//! Test từ chối đường dẫn archive không an toàn.
#![allow(clippy::unwrap_used)]

use super::super::super::super::{archive_path, archive_policy};

#[test]
fn invalid_digest_is_rejected_before_cache_creation() {
    let root = tempfile::tempdir().unwrap();
    assert!(archive_path::archive_path(root.path(), "../escape").is_err());
    assert!(archive_path::archive_path(root.path(), &"0".repeat(63)).is_err());
    assert!(!root.path().join(archive_policy::ARCHIVE_DIRECTORY).exists());
}

#[cfg(unix)]
#[test]
fn linked_cache_directory_is_rejected() {
    let root = tempfile::tempdir().unwrap();
    let outside = tempfile::tempdir().unwrap();
    let digest = "a".repeat(64);
    std::os::unix::fs::symlink(
        outside.path(),
        root.path().join(archive_policy::ARCHIVE_DIRECTORY),
    )
    .unwrap();
    assert!(archive_path::archive_path(root.path(), &digest).is_err());
}
