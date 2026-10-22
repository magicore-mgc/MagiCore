//! Tests for this runtime-detection owner.

use crate::commands::optimizer::runtime_detect::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_detect_lib_go() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("go.mod"), "module example.com/lib").unwrap();

    let runtimes = detect_runtimes(dir.path(), "lib");
    assert_eq!(runtimes, vec![DetectedRuntime::GoLib]);
}

#[test]
fn test_detect_implicit_rust_lib_target() {
    let dir = TempDir::new().unwrap();
    fs::create_dir_all(dir.path().join("src")).unwrap();
    fs::write(
        dir.path().join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n",
    )
    .unwrap();
    fs::write(dir.path().join("src/lib.rs"), "pub fn demo() {}\n").unwrap();

    let runtimes = detect_runtimes(dir.path(), "lib");
    assert_eq!(runtimes, vec![DetectedRuntime::RustLib]);
}

#[test]
fn test_binary_only_rust_project_is_not_detected_as_lib() {
    let dir = TempDir::new().unwrap();
    fs::create_dir_all(dir.path().join("src")).unwrap();
    fs::write(
        dir.path().join("Cargo.toml"),
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\n",
    )
    .unwrap();
    fs::write(dir.path().join("src/main.rs"), "fn main() {}\n").unwrap();

    let runtimes = detect_runtimes(dir.path(), "lib");
    assert_eq!(runtimes, vec![DetectedRuntime::Unknown]);
}
