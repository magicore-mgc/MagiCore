//! Tests for AI native-Python dependency-lifecycle admission.
//! Test điều kiện nhận dependency lifecycle Python native của AI.

use super::super::require_native_ai_python;

#[test]
fn native_python_manifest_is_admitted() {
    let project = tempfile::tempdir().expect("tempdir");
    std::fs::write(
        project.path().join("pyproject.toml"),
        "[project]\nname = \"demo\"\nversion = \"0.1.0\"\n",
    )
    .expect("write native Python manifest");

    require_native_ai_python(project.path(), "install").expect("native lane is supported");
}

#[test]
fn foreign_python_lock_is_refused_without_an_external_runner() {
    let project = tempfile::tempdir().expect("tempdir");
    std::fs::write(
        project.path().join("pyproject.toml"),
        "[project]\nname = \"demo\"\nversion = \"0.1.0\"\n",
    )
    .expect("write supported Python manifest");
    std::fs::write(project.path().join("uv.lock"), "version = 1\n").expect("write foreign lock");

    let error = require_native_ai_python(project.path(), "install").unwrap_err();
    assert!(
        error
            .to_string()
            .contains("No external package manager was invoked")
    );
}
