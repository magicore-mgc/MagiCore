//! Tests mutation-policy failures at the shared writer boundary.
//! Kiểm tra lỗi policy mutation tại boundary writer dùng chung.

use super::super::ensure_lockfile_mutation_allowed;
use crate::LockfileError;

#[test]
fn missing_lock_target_is_allowed_for_first_write() {
    let directory = tempfile::tempdir().expect("test operation must succeed");
    let lock_path = directory.path().join("mgc.lock");
    assert!(ensure_lockfile_mutation_allowed(&lock_path).is_ok());
}

#[test]
fn signature_artifact_blocks_ordinary_mutation() {
    let directory = tempfile::tempdir().expect("test operation must succeed");
    let lock_path = directory.path().join("mgc.lock");
    std::fs::write(&lock_path, "version = \"3\"\n[metadata]\n")
        .expect("test operation must succeed");
    std::fs::write(directory.path().join("mgc.lock.sig"), "signature")
        .expect("test operation must succeed");

    assert!(matches!(
        ensure_lockfile_mutation_allowed(&lock_path),
        Err(LockfileError::SignedLockMutation(_))
    ));
}

#[test]
fn non_regular_lock_target_is_rejected() {
    let directory = tempfile::tempdir().expect("test operation must succeed");
    assert!(matches!(
        ensure_lockfile_mutation_allowed(directory.path()),
        Err(LockfileError::WriteFailed(message)) if message.contains("non-regular")
    ));
}

#[test]
fn v4_lock_is_not_written_through_the_legacy_policy() {
    let directory = tempfile::tempdir().expect("test operation must succeed");
    let lock_path = directory.path().join("mgc.lock");
    let mut document = crate::canonical::LockfileV4::new("mgc/test");
    let bytes =
        crate::writer::finalize_v4_document(&mut document).expect("test operation must succeed");
    std::fs::write(&lock_path, bytes).expect("test operation must succeed");

    assert!(matches!(
        ensure_lockfile_mutation_allowed(&lock_path),
        Err(LockfileError::WriteFailed(message)) if message.contains("cannot mutate v4")
    ));
}
