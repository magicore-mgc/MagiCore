//! Bounded, no-follow reads for ecosystem lock inputs — kiểm tra đọc lock an toàn.

use mgc_lockfile::read_bounded_regular_file;
use std::fs;
use tempfile::tempdir;

#[test]
fn bounded_reader_accepts_regular_file_within_limit() {
    let dir = tempdir().expect("test operation must succeed");
    let path = dir.path().join("packages.lock.json");
    fs::write(&path, b"{\"dependencies\":{}} ").expect("test operation must succeed");

    let bytes =
        read_bounded_regular_file(&path, 64, "package lock").expect("test operation must succeed");

    assert_eq!(bytes, b"{\"dependencies\":{}} ");
}

#[test]
fn bounded_reader_rejects_file_over_limit() {
    let dir = tempdir().expect("test operation must succeed");
    let path = dir.path().join("oversized.lock");
    fs::write(&path, b"12345").expect("test operation must succeed");

    let error =
        read_bounded_regular_file(&path, 4, "package lock").expect_err("test operation must fail");

    assert!(error.to_string().contains("too large"));
}

#[test]
fn bounded_reader_rejects_overflowing_limit() {
    let dir = tempdir().expect("test operation must succeed");
    let path = dir.path().join("small.lock");
    fs::write(&path, b"x").expect("test operation must succeed");

    let error = read_bounded_regular_file(&path, u64::MAX, "package lock")
        .expect_err("test operation must fail");

    assert!(error.to_string().contains("limit is too large"));
}

#[cfg(unix)]
#[test]
fn bounded_reader_rejects_symlink() {
    let dir = tempdir().expect("test operation must succeed");
    let target = dir.path().join("outside.lock");
    let link = dir.path().join("packages.lock.json");
    fs::write(&target, b"{}\n").expect("test operation must succeed");
    std::os::unix::fs::symlink(&target, &link).expect("test operation must succeed");

    let error =
        read_bounded_regular_file(&link, 64, "package lock").expect_err("test operation must fail");

    assert!(error.to_string().contains("regular non-symlink"));
    assert_eq!(
        fs::read(target).expect("test operation must succeed"),
        b"{}\n"
    );
}

#[test]
fn bounded_reader_rejects_directory() {
    let dir = tempdir().expect("test operation must succeed");

    let error = read_bounded_regular_file(dir.path(), 64, "package lock")
        .expect_err("test operation must fail");

    assert!(error.to_string().contains("regular non-symlink"));
}
