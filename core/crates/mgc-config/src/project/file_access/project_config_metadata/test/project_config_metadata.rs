//! Tests for the regular project-config metadata guard.
//! Test guard metadata config project thường.
use super::super::ensure_regular_project_config;

#[test]
fn missing_config_returns_none_and_regular_config_returns_metadata() {
    let project = tempfile::tempdir().expect("temporary project should be created");
    let path = project.path().join("mgc.toml");

    assert!(ensure_regular_project_config(&path).unwrap().is_none());
    std::fs::write(&path, "ecosystem = 'web'\n").expect("config should be written");
    assert!(
        ensure_regular_project_config(&path)
            .unwrap()
            .unwrap()
            .is_file()
    );
}

#[test]
fn directory_config_is_rejected() {
    let project = tempfile::tempdir().expect("temporary project should be created");
    let error = ensure_regular_project_config(project.path())
        .expect_err("directory config must be rejected");

    assert!(error.to_string().contains("must be a regular file"));
}

#[cfg(unix)]
#[test]
fn symlink_config_is_rejected() {
    use std::os::unix::fs::symlink;

    let project = tempfile::tempdir().expect("temporary project should be created");
    let target = project.path().join("target.toml");
    let link = project.path().join("mgc.toml");
    std::fs::write(&target, "ecosystem = 'web'\n").expect("target should be written");
    symlink(&target, &link).expect("symlink should be created");

    let error =
        ensure_regular_project_config(&link).expect_err("symlink config must not be accepted");

    assert!(error.to_string().contains("must not be a symlink"));
}

#[cfg(windows)]
#[test]
fn reparse_config_is_rejected() {
    use std::os::windows::fs::symlink_file;

    let project = tempfile::tempdir().expect("temporary project should be created");
    let target = project.path().join("target.toml");
    let link = project.path().join("mgc.toml");
    std::fs::write(&target, "ecosystem = 'web'\n").expect("target should be written");
    symlink_file(&target, &link).expect("file symlink should be created");

    let error = ensure_regular_project_config(&link).expect_err("reparse config must be rejected");

    assert!(error.to_string().contains("must not be a symlink"));
}
