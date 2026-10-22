//! Rust Cargo manifest mapping and fallback tests.
//! Test mapping và fallback manifest Cargo của Rust.

#![allow(clippy::unwrap_used)]

use super::super::read_cargo_manifest;

#[test]
fn maps_path_and_version_dependencies() {
    let root = tempfile::tempdir().unwrap();
    let manifest_path = root.path().join("Cargo.toml");
    std::fs::write(
        &manifest_path,
        r#"[package]
name = "core-lib"
version = "0.1.0"
[dependencies]
common = { path = "../common" }
serde = "1.0"
"#,
    )
    .unwrap();

    let manifest = read_cargo_manifest(&manifest_path).unwrap().unwrap();

    assert_eq!(manifest.name, "core-lib");
    assert_eq!(
        manifest.dependencies.get("common").map(String::as_str),
        Some("workspace:*")
    );
    assert_eq!(
        manifest.dependencies.get("serde").map(String::as_str),
        Some("1.0")
    );
}

#[test]
fn malformed_or_unrecognized_toml_returns_none() {
    let root = tempfile::tempdir().unwrap();
    let manifest_path = root.path().join("Cargo.toml");
    std::fs::write(&manifest_path, "[package\nname = \"broken\"").unwrap();

    assert!(read_cargo_manifest(&manifest_path).unwrap().is_none());
}
