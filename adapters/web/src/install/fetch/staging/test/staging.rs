#![allow(clippy::unwrap_used)]
//! Staging owner contract tests. / Kiểm thử contract owner staging.

use super::super::*;

#[test]
fn staging_paths_are_unique_and_owned_file_is_removed_on_drop() {
    let root = tempfile::tempdir().unwrap();
    let final_path = root.path().join("package.tgz");
    let first_path = unique_tarball_staging_path(&final_path);
    let second_path = unique_tarball_staging_path(&final_path);

    assert_ne!(first_path, second_path);
    assert_eq!(first_path.parent(), final_path.parent());

    std::fs::write(&first_path, b"in-flight package").unwrap();
    let guard = DownloadStagingGuard::new(first_path.clone());
    assert_eq!(guard.path(), first_path.as_path());
    drop(guard);

    assert!(!first_path.exists());
}
