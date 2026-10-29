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

/// A Python AI project gets the PYTHON lane, never the PyTorch lane.
///
/// The property this test protects is "not misclassified as PyTorch" — not
/// "unknown". `Unknown` was the old answer and it meant `mgc optimizer` refused
/// to run for every AI project that did not import an ML library, including the
/// two frameworks MagiCore itself scaffolds (`python-agent`, `mcp-server`).
/// (Thuộc tính cần bảo vệ là "không bị nhầm thành PyTorch" — không phải
/// "unknown". `Unknown` là câu trả lời cũ và khiến `mgc optimizer` từ chối chạy
/// cho mọi project AI không import thư viện ML, kể cả hai framework MagiCore tự
/// scaffold.)
#[test]
fn test_generic_python_ai_gets_the_python_lane_not_pytorch() {
    let dir = TempDir::new().unwrap();
    fs::write(
        dir.path().join("pyproject.toml"),
        "[project]\ndependencies = [\"tensorflow>=2\"]\n",
    )
    .unwrap();

    let runtimes = detect_runtimes(dir.path(), "ai");
    assert!(
        !runtimes.contains(&DetectedRuntime::PythonPyTorch),
        "tensorflow is not torch: {runtimes:?}"
    );
    assert!(
        runtimes.contains(&DetectedRuntime::PythonLib),
        "a Python AI project needs a Python runtime to be optimized: {runtimes:?}"
    );
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

/// The word "torch" in a project NAME does not enable the PyTorch lane.
///
/// It does select the generic Python lane, because the project is still Python.
/// (Chữ "torch" trong TÊN project không bật lane PyTorch — nhưng project vẫn là
/// Python nên chọn lane Python chung.)
#[test]
fn test_unrelated_torch_metadata_does_not_enable_pytorch_optimizer() {
    let dir = TempDir::new().unwrap();
    fs::write(
        dir.path().join("pyproject.toml"),
        "[project]\nname = \"torch-documentation-example\"\ndependencies = [\"numpy\"]\n",
    )
    .unwrap();

    let runtimes = detect_runtimes(dir.path(), "ai");
    assert!(
        !runtimes.contains(&DetectedRuntime::PythonPyTorch),
        "a name is not a dependency: {runtimes:?}"
    );
    assert!(runtimes.contains(&DetectedRuntime::PythonLib));
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

// ── Plain Python AI projects must be detected ───────────────────────────
// MagiCore's own AI frameworks (`python-agent`, `mcp-server`) import no ML
// library, so a detection that only looks for torch/candle/tensorflow reported
// "No runtime detected" for a project MagiCore had just scaffolded.
// (Framework AI của MagiCore không import thư viện ML nào, nên detection chỉ tìm
// torch/candle/tensorflow báo "không có runtime" cho project do chính nó
// scaffold.)

fn project(files: &[(&str, &str)]) -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("temp dir");
    for (name, body) in files {
        std::fs::write(dir.path().join(name), body).expect("write");
    }
    dir
}

/// A `pyproject.toml` with no ML dependency still selects a Python runtime.
/// (`pyproject.toml` không có dependency ML vẫn chọn runtime Python.)
#[test]
fn a_plain_python_ai_project_selects_the_python_runtime() {
    let dir = project(&[(
        "pyproject.toml",
        "[project]\nname = \"agent\"\nversion = \"0.1.0\"\n",
    )]);
    let detected: Vec<String> = detect_ai_runtime(dir.path())
        .iter()
        .map(|r| format!("{r:?}"))
        .collect();
    assert!(
        detected.contains(&"PythonLib".to_string()),
        "a Python AI project must get a Python runtime so the optimizer runs: {detected:?}"
    );
}

/// An ML-specific library still wins its own lane, so a torch project is not
/// downgraded to the generic Python lane.
/// (Thư viện ML vẫn giữ lane riêng, không bị hạ xuống lane Python chung.)
#[test]
fn an_ml_project_keeps_its_specific_runtime() {
    let dir = project(&[(
        "pyproject.toml",
        "[project]\nname = \"m\"\nversion = \"0.1.0\"\ndependencies = [\"torch>=2\"]\n",
    )]);
    let detected: Vec<String> = detect_ai_runtime(dir.path())
        .iter()
        .map(|r| format!("{r:?}"))
        .collect();
    assert!(
        detected.contains(&"PythonPyTorch".to_string()),
        "torch must keep the PyTorch lane: {detected:?}"
    );
    assert!(
        !detected.contains(&"PythonLib".to_string()),
        "the generic fallback must not duplicate a specific lane: {detected:?}"
    );
}

/// A directory with no Python manifest at all still detects nothing, so the
/// optimizer keeps reporting "nothing was optimized" for a non-project.
/// (Thư mục không có manifest Python vẫn không detect gì, để optimizer tiếp tục
/// báo trung thực.)
#[test]
fn a_non_project_still_detects_nothing() {
    let dir = project(&[("README.md", "# notes")]);
    assert!(detect_ai_runtime(dir.path()).is_empty());
}

/// A `setup.py`-only AI project is still Python.
/// (Project AI chỉ có `setup.py` vẫn là Python.)
#[test]
fn a_setup_py_ai_project_selects_the_python_runtime() {
    let dir = project(&[("setup.py", "from setuptools import setup\nsetup()\n")]);
    assert!(detect_ai_runtime(dir.path()).contains(&DetectedRuntime::PythonLib));
}

/// A `pyproject.toml` that is a symlink escaping the project is refused.
///
/// The fallback must read through `project_text`, which rejects escaping links;
/// `Path::exists()` would follow the link and let a planted manifest outside the
/// tree select this project's runtime.
/// (Fallback phải đọc qua `project_text` để từ chối symlink trỏ ra ngoài;
/// `Path::exists()` sẽ theo link.)
#[test]
fn a_planted_manifest_symlink_is_not_followed() {
    let outside = tempfile::tempdir().expect("outside");
    std::fs::write(
        outside.path().join("pyproject.toml"),
        "[project]\nname = \"planted\"\n",
    )
    .expect("write");
    let project_dir = tempfile::tempdir().expect("project");
    #[cfg(unix)]
    std::os::unix::fs::symlink(
        outside.path().join("pyproject.toml"),
        project_dir.path().join("pyproject.toml"),
    )
    .expect("symlink");

    #[cfg(unix)]
    assert!(
        detect_ai_runtime(project_dir.path()).is_empty(),
        "an escaping manifest symlink must not select a runtime"
    );
}
