// Existing v4 policy test cases: file verification.
// Test policy V4 hiện có: file verification.

#[cfg(unix)]
#[test]
fn v4_file_verifier_refuses_symlinked_lockfiles() {
    let dir = tempfile::tempdir().expect("test operation must succeed");
    let real_lock = dir.path().join("real.lock");
    let linked_lock = dir.path().join("mgc.lock");
    let lock = unsigned_doc();
    std::fs::write(
        &real_lock,
        mgc_lockfile::canonical::write_v4_document(&lock).expect("test operation must succeed"),
    )
    .expect("test operation must succeed");
    std::os::unix::fs::symlink(&real_lock, &linked_lock).expect("test operation must succeed");

    let result = verify_v4_file(&linked_lock, LockPolicyMode::Warn, &[]);

    assert!(
        result.is_err(),
        "verifier must not follow a lockfile symlink"
    );
}

#[test]
fn v4_file_verifier_rejects_oversized_lockfiles() {
    let dir = tempfile::tempdir().expect("test operation must succeed");
    let lock_path = dir.path().join("mgc.lock");
    let mut lock = unsigned_doc();
    lock.metadata.generator = "x".repeat(10 * 1024 * 1024 + 1);
    lock.metadata.lockfile_hash = payload_digest(&lock.payload());
    std::fs::write(
        &lock_path,
        mgc_lockfile::canonical::write_v4_document(&lock).expect("test operation must succeed"),
    )
    .expect("test operation must succeed");

    let result = verify_v4_file(&lock_path, LockPolicyMode::Warn, &[]);

    assert!(
        result.is_err(),
        "verifier must enforce the lockfile size cap"
    );
}
