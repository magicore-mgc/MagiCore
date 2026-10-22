//! External Python dependency-manifest tests.
//! Test manifest dependency Python bên ngoài.

use super::super::external_python_manifest_name;

#[test]
fn recognizes_foreign_lock_and_requirements_files_without_parsing_them() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(project.path().join("uv.lock"), "arbitrary bytes\n").unwrap();
    assert_eq!(
        crate::install::external_python_manifest_name(project.path()).unwrap(),
        Some("uv.lock".to_string())
    );
    std::fs::remove_file(project.path().join("uv.lock")).unwrap();
    std::fs::write(
        project.path().join("requirements-Dev.TXT"),
        "arbitrary bytes\n",
    )
    .unwrap();
    assert_eq!(
        crate::install::external_python_manifest_name(project.path()).unwrap(),
        Some("requirements-dev.txt".to_string())
    );
}

#[test]
fn returns_none_when_no_foreign_manifest_is_present() {
    let project = tempfile::tempdir().unwrap();
    assert_eq!(external_python_manifest_name(project.path()).unwrap(), None);
}

#[test]
fn keeps_directory_read_failures_in_the_existing_error_category() {
    let project = tempfile::tempdir().unwrap();
    let missing = project.path().join("missing");
    let error = crate::install::external_python_manifest_name(&missing).unwrap_err();
    assert!(matches!(&error, mgc_types::MgError::Other(_)));
    assert!(
        error
            .to_string()
            .contains("cannot inspect Python dependency manifests")
    );
}
