//! Tests for Lib manifest classification.
//! Test phân loại manifest Lib.

use super::super::manifest_is_lib;
use std::fs;

#[test]
fn explicit_ecosystem_and_lib_section_are_authoritative() {
    let project = tempfile::tempdir().expect("temporary project");
    fs::write(project.path().join("mgc.toml"), "ecosystem = \"lib\"\n")
        .expect("write ecosystem config");
    assert!(manifest_is_lib(project.path()));

    fs::write(project.path().join("mgc.toml"), "[lib]\n").expect("write lib section");
    assert!(manifest_is_lib(project.path()));
}

#[test]
fn supported_metadata_probe_can_classify_lib_project() {
    let project = tempfile::tempdir().expect("temporary project");
    fs::write(
        project.path().join("package.json"),
        "{\"magicore\":{\"core\":\"lib\"}}\n",
    )
    .expect("write package metadata");
    assert!(manifest_is_lib(project.path()));
}

#[test]
fn unknown_or_missing_metadata_does_not_classify_as_lib() {
    let project = tempfile::tempdir().expect("temporary project");
    fs::write(
        project.path().join("package.json"),
        "{\"magicore\":{\"core\":\"web\"}}\n",
    )
    .expect("write foreign metadata");
    assert!(!manifest_is_lib(project.path()));

    let empty = tempfile::tempdir().expect("empty project");
    assert!(!manifest_is_lib(empty.path()));
}
