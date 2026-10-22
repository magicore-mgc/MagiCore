//! Behavior tests for bounded no-follow project-text reads.
//! Test hành vi đọc project text bounded, no-follow.
use super::super::read_regular_project_text;

#[test]
fn missing_path_returns_none_and_regular_file_returns_text() {
    let project = tempfile::tempdir().expect("temporary project should be created");
    let path = project.path().join("mgc.toml");

    assert_eq!(
        read_regular_project_text(&path, "project config").unwrap(),
        None
    );
    std::fs::write(&path, "ecosystem = 'web'\n").expect("config should be written");
    assert_eq!(
        read_regular_project_text(&path, "project config")
            .unwrap()
            .as_deref(),
        Some("ecosystem = 'web'\n")
    );
}

#[test]
fn directory_path_is_rejected_as_non_regular() {
    let project = tempfile::tempdir().expect("temporary project should be created");
    let error = read_regular_project_text(project.path(), "project config")
        .expect_err("directory must not be read as text");

    assert_eq!(
        error.to_string(),
        format!(
            "project config '{}' must be a regular file",
            project.path().display()
        )
    );
}

#[test]
fn content_over_ten_mib_is_rejected() {
    const LIMIT: usize = 10 * 1024 * 1024;
    let project = tempfile::tempdir().expect("temporary project should be created");
    let path = project.path().join("large.toml");
    std::fs::write(&path, vec![b'x'; LIMIT + 1]).expect("oversized file should be written");

    let error = read_regular_project_text(&path, "project config")
        .expect_err("oversized project text must be rejected");

    assert!(
        error
            .to_string()
            .contains("exceeds the 10485760 byte safety limit")
    );
}

#[cfg(unix)]
#[test]
fn symlink_is_rejected_before_read() {
    use std::os::unix::fs::symlink;

    let project = tempfile::tempdir().expect("temporary project should be created");
    let target = project.path().join("target.toml");
    let link = project.path().join("linked.toml");
    std::fs::write(&target, "ecosystem = 'web'\n").expect("target should be written");
    symlink(&target, &link).expect("symlink should be created");

    let error = read_regular_project_text(&link, "project config")
        .expect_err("symlink must not be followed");

    assert!(error.to_string().contains("must not be a symlink"));
}

#[cfg(windows)]
#[test]
fn reparse_point_is_rejected_before_read() {
    use std::os::windows::fs::symlink_file;

    let project = tempfile::tempdir().expect("temporary project should be created");
    let target = project.path().join("target.toml");
    let link = project.path().join("linked.toml");
    std::fs::write(&target, "ecosystem = 'web'\n").expect("target should be written");
    symlink_file(&target, &link).expect("file symlink should be created");

    let error = read_regular_project_text(&link, "project config")
        .expect_err("reparse point must not be opened as regular text");

    assert!(error.to_string().contains("must not be a symlink"));
}
