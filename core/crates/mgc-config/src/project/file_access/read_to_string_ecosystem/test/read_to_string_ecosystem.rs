//! Tests for safe `mgc.toml` loading and ecosystem extraction.
//! Test đọc an toàn `mgc.toml` và lấy ecosystem.
use super::super::read_to_string_ecosystem;

#[test]
fn missing_config_returns_none() {
    let project = tempfile::tempdir().expect("temporary project should be created");

    assert_eq!(read_to_string_ecosystem(project.path()).unwrap(), None);
}

#[test]
fn valid_config_returns_top_level_ecosystem() {
    let project = tempfile::tempdir().expect("temporary project should be created");
    std::fs::write(
        project.path().join("mgc.toml"),
        "name = 'sample'\necosystem = 'app'\n",
    )
    .expect("config should be written");

    assert_eq!(
        read_to_string_ecosystem(project.path()).unwrap().as_deref(),
        Some("app")
    );
}

#[cfg(unix)]
#[test]
fn linked_config_is_rejected_by_the_composed_reader() {
    use std::os::unix::fs::symlink;

    let project = tempfile::tempdir().expect("temporary project should be created");
    let target = project.path().join("target.toml");
    std::fs::write(&target, "ecosystem = 'web'\n").expect("target should be written");
    symlink(&target, project.path().join("mgc.toml")).expect("symlink should be created");

    let error = read_to_string_ecosystem(project.path())
        .expect_err("linked project config must fail closed");

    assert!(error.to_string().contains("must not be a symlink"));
}

#[cfg(windows)]
#[test]
fn reparse_config_is_rejected_by_the_composed_reader() {
    use std::os::windows::fs::symlink_file;

    let project = tempfile::tempdir().expect("temporary project should be created");
    let target = project.path().join("target.toml");
    std::fs::write(&target, "ecosystem = 'web'\n").expect("target should be written");
    symlink_file(&target, project.path().join("mgc.toml")).expect("file symlink should be created");

    let error = read_to_string_ecosystem(project.path())
        .expect_err("reparse project config must fail closed");

    assert!(error.to_string().contains("must not be a symlink"));
}
