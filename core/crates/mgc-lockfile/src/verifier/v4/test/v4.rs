//! Checks unsigned v4 status and legacy sidecar rejection.
//! Kiểm tra trạng thái v4 unsigned và từ chối sidecar legacy.

use super::super::verify_v4_lockfile;
use crate::{VerificationStatus, canonical};
use tempfile::tempdir;

fn unsigned_lock() -> canonical::LockfileV4 {
    let mut lock = canonical::LockfileV4::new("mgc/test");
    lock.metadata.lockfile_hash = canonical::payload_digest(&lock.payload());
    lock
}

#[test]
fn valid_unsigned_v4_document_is_unsigned() {
    let directory = tempdir().expect("test operation must succeed");
    let path = directory.path().join("mgc.lock");
    let lock = unsigned_lock();
    std::fs::write(
        &path,
        canonical::write_v4_document(&lock).expect("test operation must succeed"),
    )
    .expect("test operation must succeed");
    assert_eq!(
        verify_v4_lockfile(&path, &lock, None).expect("test operation must succeed"),
        VerificationStatus::Unsigned
    );
}

#[test]
fn legacy_sidecar_entry_invalidates_v4_verification() {
    let directory = tempdir().expect("test operation must succeed");
    let path = directory.path().join("mgc.lock");
    let lock = unsigned_lock();
    std::fs::write(
        &path,
        canonical::write_v4_document(&lock).expect("test operation must succeed"),
    )
    .expect("test operation must succeed");
    std::fs::write(path.with_extension("lock.sig"), b"anything")
        .expect("test operation must succeed");
    assert!(matches!(
        verify_v4_lockfile(&path, &lock, None).expect("test operation must succeed"),
        VerificationStatus::InvalidSignature(message)
            if message.contains("legacy signature sidecar")
    ));
}
