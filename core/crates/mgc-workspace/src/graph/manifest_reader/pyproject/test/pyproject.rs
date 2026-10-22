//! Python pyproject field and fallback tests.
//! Test field và fallback pyproject của Python.

#![allow(clippy::unwrap_used)]

use super::super::read_pyproject_manifest;

#[test]
fn reads_project_name_and_leaves_dependency_maps_empty() {
    let root = tempfile::tempdir().unwrap();
    let manifest_path = root.path().join("pyproject.toml");
    std::fs::write(
        &manifest_path,
        "[project]\nname = \"ai-agent\"\ndependencies = [\"numpy\"]\n",
    )
    .unwrap();

    let manifest = read_pyproject_manifest(&manifest_path).unwrap().unwrap();

    assert_eq!(manifest.name, "ai-agent");
    assert!(manifest.dependencies.is_empty());
}

#[test]
fn malformed_or_unrecognized_toml_returns_none() {
    let root = tempfile::tempdir().unwrap();
    let manifest_path = root.path().join("pyproject.toml");
    std::fs::write(&manifest_path, "[project\nname = \"broken\"").unwrap();

    assert!(read_pyproject_manifest(&manifest_path).unwrap().is_none());
}
