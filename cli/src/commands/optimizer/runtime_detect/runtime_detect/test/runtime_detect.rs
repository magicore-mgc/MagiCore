//! Tests for core dispatch and optimizer aggregation.

use crate::commands::optimizer::runtime_detect::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_unknown_fallback() {
    let dir = TempDir::new().unwrap();
    let runtimes = detect_runtimes(dir.path(), "web");
    assert_eq!(runtimes, vec![DetectedRuntime::Unknown]);
}

#[test]
fn optimizer_detects_web_native_languages_without_changing_test_runtime_detection() {
    let cases = [
        (
            "Cargo.toml",
            "[package]\nname = \"web-rust\"\nversion = \"0.1.0\"\n",
            DetectedRuntime::RustLib,
        ),
        (
            "go.mod",
            "module\texample.com/web-go\n\ngo 1.21\n",
            DetectedRuntime::GoLib,
        ),
        (
            "pyproject.toml",
            "[project]\nname = \"web-python\"\nversion = \"0.1.0\"\ndependencies = []\n",
            DetectedRuntime::PythonLib,
        ),
    ];

    for (manifest, content, expected) in cases {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join(manifest), content).unwrap();

        assert_eq!(
            detect_runtimes(dir.path(), "web"),
            vec![DetectedRuntime::Unknown]
        );
        assert_eq!(detect_optimizer_runtimes(dir.path(), "web"), vec![expected]);
    }
}
