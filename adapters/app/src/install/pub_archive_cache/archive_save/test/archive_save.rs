//! Archive publication rejection cases.
//! Các ca từ chối publish archive.
#![allow(clippy::unwrap_used)]

use super::super::super::super::{archive_policy, archive_save};

#[test]
fn invalid_digest_and_wrong_bytes_do_not_publish_cache() {
    let root = tempfile::tempdir().unwrap();
    assert!(archive_save::save_verified(root.path(), "../escape", b"payload").is_err());
    assert!(archive_save::save_verified(root.path(), &"0".repeat(64), b"payload").is_err());
    assert!(!root.path().join(archive_policy::ARCHIVE_DIRECTORY).exists());
}
