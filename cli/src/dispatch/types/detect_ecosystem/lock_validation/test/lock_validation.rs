//! Project lock syntax contract.

use super::super::validate;

#[test]
fn absent_lock_is_valid_for_legacy_project_detection() {
    let project = tempfile::tempdir().unwrap();
    assert!(validate(project.path()).is_ok());
}

#[test]
fn malformed_lock_returns_an_error_before_manifest_fallback() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(project.path().join("mgc.lock"), "not a lockfile\n").unwrap();

    assert!(validate(project.path()).is_err());
}
