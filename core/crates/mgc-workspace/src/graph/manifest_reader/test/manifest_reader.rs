//! Workspace manifest priority and fallback tests.
//! Test thứ tự ưu tiên và fallback manifest workspace.

#![allow(clippy::unwrap_used)]

use super::super::read_package_manifest;

#[test]
fn missing_manifest_files_return_none() {
    let root = tempfile::tempdir().unwrap();

    assert!(read_package_manifest(root.path()).unwrap().is_none());
}

#[test]
fn package_json_has_priority_over_cargo_manifest() {
    let root = tempfile::tempdir().unwrap();
    std::fs::write(root.path().join("package.json"), r#"{"name":"node-first"}"#).unwrap();
    std::fs::write(
        root.path().join("Cargo.toml"),
        "[package]\nname = \"rust-second\"\nversion = \"0.1.0\"\n",
    )
    .unwrap();

    let manifest = read_package_manifest(root.path()).unwrap().unwrap();

    assert_eq!(manifest.name, "node-first");
}

#[test]
fn invalid_cargo_manifest_falls_through_to_pyproject() {
    let root = tempfile::tempdir().unwrap();
    std::fs::write(
        root.path().join("Cargo.toml"),
        "[package\nname = \"broken\"",
    )
    .unwrap();
    std::fs::write(
        root.path().join("pyproject.toml"),
        "[project]\nname = \"python-after-cargo\"\n",
    )
    .unwrap();

    let manifest = read_package_manifest(root.path()).unwrap().unwrap();

    assert_eq!(manifest.name, "python-after-cargo");
}

#[test]
fn malformed_package_json_keeps_original_json_error_type() {
    let root = tempfile::tempdir().unwrap();
    std::fs::write(root.path().join("package.json"), "{").unwrap();
    std::fs::write(
        root.path().join("Cargo.toml"),
        "[package]\nname = \"must-not-fallback\"\nversion = \"0.1.0\"\n",
    )
    .unwrap();

    let error = read_package_manifest(root.path()).unwrap_err();

    assert!(error.downcast_ref::<serde_json::Error>().is_some());
}
