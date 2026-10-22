//! Tests for pyproject Lib identity probing.
//! Test probe identity Lib trong pyproject.

use super::super::probe_pyproject;
use std::fs;

#[test]
fn reads_tool_magicore_core_string() {
    let project = tempfile::tempdir().expect("temporary project");
    let path = project.path().join("pyproject.toml");
    fs::write(&path, "[tool.magicore]\ncore = \"lib\"\n").expect("write Python metadata");

    assert_eq!(probe_pyproject(&path).as_deref(), Some("lib"));
}

#[test]
fn malformed_toml_returns_no_match() {
    let project = tempfile::tempdir().expect("temporary project");
    let path = project.path().join("pyproject.toml");
    fs::write(&path, "[tool.magicore\n").expect("write malformed Python metadata");

    assert_eq!(probe_pyproject(&path), None);
}
