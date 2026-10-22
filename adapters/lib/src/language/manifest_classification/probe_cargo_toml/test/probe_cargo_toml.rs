//! Tests for Cargo Lib identity probing.
//! Test probe identity Lib trong Cargo.

use super::super::probe_cargo_toml;
use std::fs;

#[test]
fn reads_package_magicore_core_string() {
    let project = tempfile::tempdir().expect("temporary project");
    let path = project.path().join("Cargo.toml");
    fs::write(
        &path,
        "[package]\nname=\"fixture\"\nversion=\"0.1.0\"\n\
         [package.metadata.magicore]\ncore=\"lib\"\n",
    )
    .expect("write Cargo metadata");

    assert_eq!(probe_cargo_toml(&path).as_deref(), Some("lib"));
}

#[test]
fn malformed_toml_returns_no_match() {
    let project = tempfile::tempdir().expect("temporary project");
    let path = project.path().join("Cargo.toml");
    fs::write(&path, "[package\n").expect("write malformed Cargo metadata");

    assert_eq!(probe_cargo_toml(&path), None);
}
