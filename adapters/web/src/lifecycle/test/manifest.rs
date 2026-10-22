//! Lifecycle behavior tests for the matching production owner.
//! Test hành vi lifecycle cho owner production tương ứng.

use super::super::LifecycleRunner;

#[test]
fn lifecycle_errors_on_invalid_package_json() {
    let _environment = super::helpers::environment_test_guard();
    let project = tempfile::tempdir().unwrap();
    let package = tempfile::tempdir().unwrap();
    std::fs::write(package.path().join("package.json"), "{not-json").unwrap();

    let err = LifecycleRunner::run_scripts(package.path(), project.path()).unwrap_err();

    assert!(
        err.to_string()
            .contains("failed to parse package.json for lifecycle"),
        "unexpected error: {err}"
    );
}
