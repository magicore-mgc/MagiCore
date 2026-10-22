#![allow(clippy::unwrap_used)]

use mgc_lockfile::{load_lockfile, read_lockfile_checked};
use tempfile::tempdir;

#[cfg(unix)]
#[test]
fn lockfile_readers_refuse_symlinks_instead_of_following_them() {
    let project = tempdir().expect("test operation must succeed");
    let outside = project.path().join("outside.lock");
    std::fs::write(&outside, "not a lockfile").expect("test operation must succeed");
    let linked = project.path().join("mgc.lock");
    std::os::unix::fs::symlink(&outside, &linked).expect("test operation must succeed");

    let checked = read_lockfile_checked(project.path()).expect_err("test operation must fail");
    assert!(checked.to_string().contains("regular non-symlink file"));

    let loaded = load_lockfile(&linked).expect_err("test operation must fail");
    assert!(loaded.to_string().contains("regular non-symlink file"));
}

#[test]
fn checked_lock_reader_reports_absence_without_following_paths() {
    let project = tempdir().expect("test operation must succeed");
    assert!(
        read_lockfile_checked(project.path())
            .expect("test operation must succeed")
            .is_none()
    );
}

/// Windows publication must work beyond the legacy MAX_PATH limit.
/// Publish Windows phải chạy được khi path dài hơn MAX_PATH cũ.
#[cfg(windows)]
#[test]
fn atomic_replace_handles_long_windows_paths() {
    let project = tempdir().expect("test operation must succeed");
    let mut directory = std::fs::canonicalize(project.path()).expect("test operation must succeed");
    for _ in 0..8 {
        directory = directory.join("package-store-long-component-1234567890");
    }
    std::fs::create_dir_all(&directory).expect("test operation must succeed");
    let staging = directory.join("staging.tmp");
    let destination = directory.join("package-marker.json");
    std::fs::write(&staging, b"first").expect("test operation must succeed");
    mgc_lockfile::atomic::atomic_replace_file(&staging, &destination)
        .expect("test operation must succeed");
    assert_eq!(
        std::fs::read(&destination).expect("test operation must succeed"),
        b"first"
    );
    std::fs::write(&staging, b"second").expect("test operation must succeed");
    mgc_lockfile::atomic::atomic_replace_file(&staging, &destination)
        .expect("test operation must succeed");
    assert_eq!(
        std::fs::read(&destination).expect("test operation must succeed"),
        b"second"
    );
    assert!(!staging.exists());
}
