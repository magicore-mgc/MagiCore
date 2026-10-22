//! Node package.json field and error tests.
//! Test mapping field và lỗi package.json của Node.

#![allow(clippy::unwrap_used)]

use super::super::read_package_json;
use std::error::Error;

#[test]
fn maps_all_existing_dependency_sections() {
    let root = tempfile::tempdir().unwrap();
    let manifest_path = root.path().join("package.json");
    std::fs::write(
        &manifest_path,
        r#"{
            "name": "web-app",
            "dependencies": {"react": "18"},
            "devDependencies": {"vite": "5"},
            "peerDependencies": {"plugin": "*"},
            "optionalDependencies": {"native": "1"}
        }"#,
    )
    .unwrap();

    let manifest = read_package_json(&manifest_path).unwrap();

    assert_eq!(manifest.name, "web-app");
    assert_eq!(
        manifest.dependencies.get("react").map(String::as_str),
        Some("18")
    );
    assert_eq!(
        manifest.dev_dependencies.get("vite").map(String::as_str),
        Some("5")
    );
    assert_eq!(
        manifest.peer_dependencies.get("plugin").map(String::as_str),
        Some("*")
    );
    assert_eq!(
        manifest
            .optional_dependencies
            .get("native")
            .map(String::as_str),
        Some("1")
    );
}

#[test]
fn malformed_json_keeps_typed_parse_source() {
    let root = tempfile::tempdir().unwrap();
    let manifest_path = root.path().join("package.json");
    std::fs::write(&manifest_path, "{").unwrap();

    let error = read_package_json(&manifest_path).unwrap_err();

    assert!(matches!(
        &error,
        super::super::super::error::ManifestReadError::PackageJson(_)
    ));
    assert!(error.source().is_some());
}
