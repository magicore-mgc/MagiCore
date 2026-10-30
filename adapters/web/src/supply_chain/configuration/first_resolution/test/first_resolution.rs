//! Lockfile-presence decision tests.
//! Test quyết định dựa trên sự hiện diện của lockfile.

use super::super::has_prior_install_decision;

#[test]
fn project_without_lockfile_is_a_first_resolution() {
    let root = tempfile::tempdir().expect("create project directory");

    assert!(!has_prior_install_decision(root.path()));
}

#[test]
fn project_with_a_lockfile_has_a_prior_decision() {
    let root = tempfile::tempdir().expect("create project directory");
    std::fs::write(root.path().join("mgc.lock"), "version = \"4\"\n").expect("write lockfile");

    assert!(has_prior_install_decision(root.path()));
}

#[test]
fn a_record_that_is_not_a_regular_file_is_not_a_decision() {
    let root = tempfile::tempdir().expect("create project directory");
    std::fs::create_dir(root.path().join("mgc.lock")).expect("create mgc.lock directory");

    assert!(!has_prior_install_decision(root.path()));
}
