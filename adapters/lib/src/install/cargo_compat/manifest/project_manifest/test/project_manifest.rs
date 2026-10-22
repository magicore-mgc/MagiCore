//! Focused tests for Cargo project-manifest parsing.
//! Test tập trung cho parse manifest dự án Cargo.

use super::{CargoManifestError, read_project_manifest};

#[test]
fn parses_the_supported_single_package_shape() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(
        project.path().join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    )
    .unwrap();

    let manifest = read_project_manifest(project.path()).unwrap();
    assert_eq!(manifest.root_name, "demo");
    assert_eq!(manifest.root_version, "0.1.0");
}

#[test]
fn reports_missing_package_with_the_existing_message() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(project.path().join("Cargo.toml"), "[dependencies]\n").unwrap();

    let error = read_project_manifest(project.path()).unwrap_err();
    assert_eq!(
        error.to_string(),
        "Cargo projection requires a package manifest"
    );
}

#[test]
fn toml_parse_failure_retains_its_underlying_source() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(project.path().join("Cargo.toml"), "[package\n").unwrap();

    let error = read_project_manifest(project.path()).unwrap_err();
    assert!(matches!(error, CargoManifestError::TomlParse(_)));
    assert!(std::error::Error::source(&error).is_some());
    assert!(
        error
            .to_string()
            .starts_with("parse Cargo.toml for Cargo projection: ")
    );
}

#[test]
fn rejects_workspace_members_with_the_existing_message() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(
        project.path().join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n[workspace]\nmembers = [\"member\"]\n",
    )
    .unwrap();

    let error = read_project_manifest(project.path()).unwrap_err();
    assert_eq!(
        error.to_string(),
        "Cargo projection does not support workspaces yet"
    );
}
