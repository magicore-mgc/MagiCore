//! Checks the public tamper boolean projection.
//! Kiểm tra phép chiếu boolean tamper public.

use super::super::is_lockfile_tampered;
use crate::Lockfile;
use tempfile::tempdir;

#[test]
fn unsigned_valid_lock_is_not_tampered() {
    let directory = tempdir().expect("test operation must succeed");
    let path = directory.path().join("mgc.lock");
    std::fs::write(
        &path,
        crate::writer::serialize_lockfile(&Lockfile::new()).expect("test operation must succeed"),
    )
    .expect("test operation must succeed");
    assert!(!is_lockfile_tampered(&path).expect("test operation must succeed"));
}
