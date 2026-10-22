//! Hardware materialization behavior tests.
//! Test hành vi materialize của Hardware.

use super::super::implementation::existing_target_directory;
use super::super::{BENCH_PKG, materialize_template};
use std::path::Path;

#[tokio::test]
async fn materialization_keeps_an_existing_target_unchanged() {
    let project = tempfile::tempdir().expect("tempdir");
    let package = project.path().join(BENCH_PKG);
    std::fs::create_dir(&package).expect("create existing package");
    let marker = package.join("user-file.txt");
    std::fs::write(&marker, "keep this file").expect("write marker");

    materialize_template(project.path(), BENCH_PKG)
        .await
        .expect("existing materialization is a no-op");

    assert_eq!(
        std::fs::read_to_string(marker).expect("read marker"),
        "keep this file"
    );
}

#[tokio::test]
async fn materialization_rejects_a_file_at_the_target_path() {
    let project = tempfile::tempdir().expect("tempdir");
    let package = project.path().join(BENCH_PKG);
    std::fs::write(&package, "user file").expect("create file at package path");

    let error = materialize_template(project.path(), BENCH_PKG)
        .await
        .expect_err("a file cannot be treated as a materialized package");

    assert!(error.to_string().contains("not a real directory"));
    assert_eq!(
        std::fs::read_to_string(package).expect("read original file"),
        "user file"
    );
}

#[cfg(unix)]
#[tokio::test]
async fn materialization_rejects_a_symlink_at_the_target_path() {
    let project = tempfile::tempdir().expect("tempdir");
    let actual = project.path().join("actual-package");
    let package = project.path().join(BENCH_PKG);
    std::fs::create_dir(&actual).expect("create linked directory");
    std::os::unix::fs::symlink(&actual, &package).expect("create package symlink");

    let error = materialize_template(project.path(), BENCH_PKG)
        .await
        .expect_err("a symlink cannot be treated as a materialized package");

    assert!(error.to_string().contains("not a real directory"));
    assert!(
        std::fs::symlink_metadata(package)
            .expect("inspect original link")
            .file_type()
            .is_symlink()
    );
}

#[test]
fn target_inspection_does_not_follow_a_directory_symlink() {
    let project = tempfile::tempdir().expect("tempdir");
    let actual = project.path().join("actual-package");
    let package = project.path().join(BENCH_PKG);
    std::fs::create_dir(&actual).expect("create linked directory");
    #[cfg(unix)]
    std::os::unix::fs::symlink(&actual, &package).expect("create package symlink");
    #[cfg(windows)]
    std::os::windows::fs::symlink_dir(&actual, &package).expect("create package symlink");

    let error = existing_target_directory(Path::new(&package))
        .expect_err("symlink metadata must reject a linked directory");

    assert!(error.to_string().contains("not a real directory"));
}
