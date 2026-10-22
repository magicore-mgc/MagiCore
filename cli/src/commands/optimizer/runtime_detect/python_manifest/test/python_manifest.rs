//! Tests for this runtime-detection owner.

use crate::commands::optimizer::runtime_detect::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn optimizer_detects_mixed_web_language_manifests() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("package.json"), "{}\n").unwrap();
    fs::write(
        dir.path().join("pyproject.toml"),
        "[project]\nname = \"mixed-web\"\nversion = \"0.1.0\"\ndependencies = []\n",
    )
    .unwrap();

    assert_eq!(
        detect_optimizer_runtimes(dir.path(), "web"),
        vec![
            DetectedRuntime::NodeJs {
                package_manager: PackageManager::Npm
            },
            DetectedRuntime::PythonLib,
        ]
    );
}

#[test]
fn optimizer_does_not_infer_python_from_empty_or_placeholder_manifests() {
    for (manifest, content) in [
        ("requirements.txt", "# no resolved requirements\n"),
        (
            "requirements.txt",
            "--index-url https://example.invalid/simple\n",
        ),
        ("setup.py", "# placeholder only\n"),
        ("setup.py", "print(\"setup(\")\n"),
        ("pyproject.toml", "[project]\ndependencies = []\n"),
    ] {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join(manifest), content).unwrap();
        assert_eq!(
            detect_optimizer_runtimes(dir.path(), "web"),
            vec![DetectedRuntime::Unknown],
            "empty {manifest} must not qualify a Python runtime"
        );
    }
}

#[test]
fn optimizer_detects_python_requirements_and_setup_projects() {
    for (manifest, content) in [
        ("requirements.txt", "requests==2.32.0\n"),
        (
            "setup.py",
            "from setuptools import setup\nsetup(name=\"web-python\")\n",
        ),
    ] {
        let dir = TempDir::new().unwrap();
        fs::write(dir.path().join(manifest), content).unwrap();
        assert_eq!(
            detect_optimizer_runtimes(dir.path(), "web"),
            vec![DetectedRuntime::PythonLib],
            "valid {manifest} should qualify Python"
        );
    }
}
