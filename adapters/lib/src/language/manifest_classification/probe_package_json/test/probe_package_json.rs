//! Tests for package.json Lib identity probing.
//! Test probe identity Lib trong package.json.

use super::super::probe_package_json;
use std::fs;

#[test]
fn reads_magicore_core_string() {
    let project = tempfile::tempdir().expect("temporary project");
    let path = project.path().join("package.json");
    fs::write(&path, "{\"magicore\":{\"core\":\"lib\"}}\n").expect("write package metadata");

    assert_eq!(probe_package_json(&path).as_deref(), Some("lib"));
}

#[test]
fn malformed_json_returns_no_match() {
    let project = tempfile::tempdir().expect("temporary project");
    let path = project.path().join("package.json");
    fs::write(&path, "{").expect("write malformed package metadata");

    assert_eq!(probe_package_json(&path), None);
}
