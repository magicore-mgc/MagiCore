//! Tests for this runtime-detection owner.

use crate::commands::optimizer::runtime_detect::*;
use std::fs;
use tempfile::TempDir;

#[cfg(unix)]
#[test]
fn runtime_detection_does_not_read_external_manifest_symlinks() {
    use std::os::unix::fs::symlink;

    let project = TempDir::new().unwrap();
    let external = TempDir::new().unwrap();
    fs::write(
        external.path().join("package.json"),
        r#"{"dependencies":{"react-native":"1"}}"#,
    )
    .unwrap();
    fs::write(
        external.path().join("pyproject.toml"),
        "[project]\ndependencies = [\"torch>=2\"]\n",
    )
    .unwrap();
    fs::write(external.path().join("requirements.txt"), "torch==2\n").unwrap();

    symlink(
        external.path().join("package.json"),
        project.path().join("package.json"),
    )
    .unwrap();
    assert_eq!(
        detect_runtimes(project.path(), "app"),
        vec![DetectedRuntime::Unknown]
    );
    assert_eq!(
        detect_runtimes(project.path(), "web"),
        vec![DetectedRuntime::Unknown]
    );

    fs::remove_file(project.path().join("package.json")).unwrap();
    symlink(
        external.path().join("pyproject.toml"),
        project.path().join("pyproject.toml"),
    )
    .unwrap();
    symlink(
        external.path().join("requirements.txt"),
        project.path().join("requirements.txt"),
    )
    .unwrap();
    assert_eq!(
        detect_runtimes(project.path(), "ai"),
        vec![DetectedRuntime::Unknown]
    );
}
