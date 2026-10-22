//! Tests for this runtime-detection owner.

use crate::commands::optimizer::runtime_detect::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_detect_ai_python_pytorch() {
    let dir = TempDir::new().unwrap();
    fs::write(
        dir.path().join("pyproject.toml"),
        "[project]\ndependencies = [\"torch>=2.8\"]\n",
    )
    .unwrap();

    let runtimes = detect_runtimes(dir.path(), "ai");
    assert_eq!(runtimes, vec![DetectedRuntime::PythonPyTorch]);
}

#[test]
fn test_generic_python_ai_is_not_misclassified_as_pytorch() {
    let dir = TempDir::new().unwrap();
    fs::write(
        dir.path().join("pyproject.toml"),
        "[project]\ndependencies = [\"tensorflow>=2\"]\n",
    )
    .unwrap();

    let runtimes = detect_runtimes(dir.path(), "ai");
    assert_eq!(runtimes, vec![DetectedRuntime::Unknown]);
}

#[test]
fn test_detect_ai_python_pytorch_from_requirements() {
    let dir = TempDir::new().unwrap();
    fs::write(
        dir.path().join("pyproject.toml"),
        "[project]\nname = \"demo\"\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("requirements.txt"),
        "# runtime\ntorch[distributed]==2.8.0 ; python_version >= '3.11'\n",
    )
    .unwrap();

    let runtimes = detect_runtimes(dir.path(), "ai");
    assert_eq!(runtimes, vec![DetectedRuntime::PythonPyTorch]);
}

#[test]
fn test_detect_ai_python_pytorch_from_poetry_dependencies() {
    let dir = TempDir::new().unwrap();
    fs::write(
        dir.path().join("pyproject.toml"),
        "[tool.poetry.dependencies]\npython = \"^3.12\"\ntorch = \"^2.8\"\n",
    )
    .unwrap();

    let runtimes = detect_runtimes(dir.path(), "ai");
    assert_eq!(runtimes, vec![DetectedRuntime::PythonPyTorch]);
}

#[test]
fn test_unrelated_torch_metadata_does_not_enable_pytorch_optimizer() {
    let dir = TempDir::new().unwrap();
    fs::write(
        dir.path().join("pyproject.toml"),
        "[project]\nname = \"torch-documentation-example\"\ndependencies = [\"numpy\"]\n",
    )
    .unwrap();

    let runtimes = detect_runtimes(dir.path(), "ai");
    assert_eq!(runtimes, vec![DetectedRuntime::Unknown]);
}

#[test]
fn test_detect_ai_rust_candle() {
    let dir = TempDir::new().unwrap();
    fs::write(
        dir.path().join("Cargo.toml"),
        "[dependencies]\ncandle-core = \"0.1\"",
    )
    .unwrap();

    let runtimes = detect_runtimes(dir.path(), "ai");
    assert_eq!(runtimes, vec![DetectedRuntime::RustCandle]);
}
