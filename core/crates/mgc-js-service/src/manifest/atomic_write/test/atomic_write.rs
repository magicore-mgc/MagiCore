//! Atomic publication safety tests.
//! Test an toàn khi publish atomic.

use super::super::atomic_write;

#[test]
fn atomic_write_creates_a_missing_file() {
    let dir = tempfile::tempdir().expect("temporary directory");
    let path = dir.path().join("state.json");

    atomic_write(&path, b"{\"ok\":true}").expect("write missing file");

    assert_eq!(std::fs::read(path).expect("read file"), b"{\"ok\":true}");
}

#[cfg(unix)]
#[test]
fn atomic_write_rejects_a_symlink_destination() {
    use std::os::unix::fs::symlink;

    let dir = tempfile::tempdir().expect("temporary directory");
    let target = dir.path().join("target.json");
    let link = dir.path().join("link.json");
    std::fs::write(&target, b"sentinel").expect("write target");
    symlink(&target, &link).expect("create symlink");

    assert!(atomic_write(&link, b"changed").is_err());
    assert_eq!(std::fs::read(target).expect("read target"), b"sentinel");
}
