//! Cargo and Python TOML core-hint extraction contracts.

use super::super::{cargo_core, python_core};

#[test]
fn reads_cargo_package_metadata_core() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(
        project.path().join("Cargo.toml"),
        "[package]\nname = \"fixture\"\nversion = \"0.1.0\"\n[package.metadata.magicore]\ncore = \"game\"\n",
    )
    .unwrap();

    assert_eq!(cargo_core(project.path()).unwrap().as_deref(), Some("game"));
}

#[test]
fn reads_python_tool_metadata_core() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(
        project.path().join("pyproject.toml"),
        "[tool.magicore]\ncore = \"ai\"\n",
    )
    .unwrap();

    assert_eq!(python_core(project.path()).unwrap().as_deref(), Some("ai"));
}

#[test]
fn malformed_cargo_toml_preserves_the_parser_error_type() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(project.path().join("Cargo.toml"), "[package\n").unwrap();

    let error = cargo_core(project.path()).unwrap_err();
    assert!(error.downcast_ref::<toml::de::Error>().is_some());
}
