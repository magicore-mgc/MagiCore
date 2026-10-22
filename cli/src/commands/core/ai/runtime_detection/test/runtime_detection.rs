//! Existing tests for AI optimizer runtime classification.
//! Test hiện có cho phân loại optimizer runtime AI.

use super::super::detect_ai_runtime;
use crate::commands::optimizer::runtime_detect::DetectedRuntime;

#[test]
fn generic_python_ai_detects_torch_from_pep621_dependencies() {
    let temp = tempfile::tempdir().expect("tempdir");
    std::fs::write(
        temp.path().join("pyproject.toml"),
        "[project]\ndependencies = [\"torch>=2.8\", \"numpy\"]\n",
    )
    .expect("pyproject");

    assert_eq!(
        detect_ai_runtime(temp.path(), "python-agent"),
        DetectedRuntime::PythonPyTorch
    );
}

#[test]
fn generic_python_ai_without_torch_remains_unknown() {
    let temp = tempfile::tempdir().expect("tempdir");
    std::fs::write(
        temp.path().join("pyproject.toml"),
        "[project]\ndependencies = [\"tensorflow>=2\"]\n",
    )
    .expect("pyproject");

    assert_eq!(
        detect_ai_runtime(temp.path(), "python-agent"),
        DetectedRuntime::Unknown
    );
}
