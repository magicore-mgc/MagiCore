//! Tests for this runtime-detection owner.

use crate::commands::optimizer::runtime_detect::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_detect_app_flutter() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("pubspec.yaml"), "name: myapp").unwrap();

    let runtimes = detect_runtimes(dir.path(), "app");
    assert_eq!(runtimes, vec![DetectedRuntime::Flutter]);
}

#[test]
fn test_detect_app_react_native() {
    let dir = TempDir::new().unwrap();
    fs::write(
        dir.path().join("package.json"),
        r#"{"dependencies": {"react-native": "0.70"}}"#,
    )
    .unwrap();

    let runtimes = detect_runtimes(dir.path(), "app");
    assert_eq!(runtimes, vec![DetectedRuntime::ReactNative]);
}
