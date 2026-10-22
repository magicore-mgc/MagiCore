//! Tests the marker detector without invoking adapter consumers.
//! Kiểm tra detector trực tiếp, không gọi consumer của adapter.

use super::super::manifest_is_any_mg;
use tempfile::tempdir;

#[test]
fn missing_manifest_marker_is_not_a_hardware_project() {
    let project = tempdir().unwrap();
    assert!(!manifest_is_any_mg(project.path()));
}

#[test]
fn regular_manifest_marker_identifies_a_hardware_project() {
    let project = tempdir().unwrap();
    std::fs::write(project.path().join("mgc.toml"), "not = [valid toml").unwrap();
    assert!(manifest_is_any_mg(project.path()));
}

#[test]
fn directory_named_manifest_marker_is_not_a_hardware_project() {
    let project = tempdir().unwrap();
    std::fs::create_dir(project.path().join("mgc.toml")).unwrap();
    assert!(!manifest_is_any_mg(project.path()));
}
