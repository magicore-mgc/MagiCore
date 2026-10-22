//! Checks public legacy verification entry-point behavior.
//! Kiểm tra hành vi entry point verify legacy public.

use super::super::{verify_lockfile, verify_lockfile_with_trust};
use crate::{Lockfile, VerificationStatus};
use tempfile::tempdir;

#[test]
fn unsigned_legacy_lock_remains_an_explicit_status() {
    let directory = tempdir().expect("test operation must succeed");
    let path = directory.path().join("mgc.lock");
    let text =
        crate::writer::serialize_lockfile(&Lockfile::new()).expect("test operation must succeed");
    std::fs::write(&path, text).expect("test operation must succeed");

    assert_eq!(
        verify_lockfile(&path).expect("test operation must succeed"),
        VerificationStatus::Unsigned
    );
    assert_eq!(
        verify_lockfile_with_trust(&path, &[]).expect("test operation must succeed"),
        VerificationStatus::Unsigned
    );
}
