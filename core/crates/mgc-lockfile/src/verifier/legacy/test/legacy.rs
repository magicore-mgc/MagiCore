//! Checks missing and malformed sidecar outcomes.
//! Kiểm tra kết quả khi thiếu hoặc sidecar sai.

use super::super::verify_legacy_lockfile;
use crate::{Lockfile, LockfileError, VerificationStatus};
use tempfile::tempdir;

#[test]
fn missing_sidecar_is_unsigned_only_for_unmarked_lock() {
    let directory = tempdir().expect("test operation must succeed");
    let path = directory.path().join("mgc.lock");
    std::fs::write(
        &path,
        crate::writer::serialize_lockfile(&Lockfile::new()).expect("test operation must succeed"),
    )
    .expect("test operation must succeed");
    assert_eq!(
        verify_legacy_lockfile(&path, None).expect("test operation must succeed"),
        VerificationStatus::Unsigned
    );
}

#[test]
fn present_malformed_sidecar_does_not_downgrade_to_unsigned() {
    let directory = tempdir().expect("test operation must succeed");
    let path = directory.path().join("mgc.lock");
    std::fs::write(
        &path,
        crate::writer::serialize_lockfile(&Lockfile::new()).expect("test operation must succeed"),
    )
    .expect("test operation must succeed");
    std::fs::write(path.with_extension("lock.sig"), b"invalid")
        .expect("test operation must succeed");
    assert!(matches!(
        verify_legacy_lockfile(&path, None),
        Err(LockfileError::InvalidSignatureFile(_))
    ));
}
