//! AI runtime recognition across Python, Rust, and Go.
use super::super::{DetectedRuntime, project_text, python_project_declares_package};
use std::path::Path;

pub(in crate::commands::optimizer) fn detect_ai_runtime(
    project_root: &Path,
) -> Vec<DetectedRuntime> {
    let mut runtimes = vec![];

    // Check declared Python/PyTorch dependencies — kiểm tra dependency Python/PyTorch đã khai báo
    if python_project_declares_package(project_root, "torch") {
        runtimes.push(DetectedRuntime::PythonPyTorch);
    }

    // Check for Rust/Candle (Cargo.toml + candle dependency) — kiểm tra Rust/Candle
    // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
    if let Some(content) = project_text(project_root, "Cargo.toml")
        && (content.contains("candle") || content.contains("burn"))
    {
        runtimes.push(DetectedRuntime::RustCandle);
    }

    // Check for Go AI (go.mod + tensorflow/onnx) — kiểm tra Go AI
    // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
    if let Some(content) = project_text(project_root, "go.mod")
        && (content.contains("tensorflow") || content.contains("onnx"))
    {
        runtimes.push(DetectedRuntime::GoTensorFlow);
    }

    // A Python AI project is a Python process regardless of which ML library it
    // imports. MagiCore's own AI frameworks are `python-agent` and `mcp-server`;
    // neither imports torch/candle/tensorflow, so the checks above returned an
    // empty list and `mgc optimizer` refused with "No runtime detected … nothing
    // was optimized" for a project MagiCore had just scaffolded itself.
    //
    // The ML-specific runtimes stay first so a torch project still gets the
    // torch lane; this entry only covers the "plain Python AI" case.
    // (Project AI Python là process Python bất kể thư viện ML nào. Hai framework
    // AI của MagiCore là `python-agent`/`mcp-server`, không import torch/candle/
    // tensorflow, nên các check trên trả rỗng và `mgc optimizer` từ chối chạy cho
    // project do chính MagiCore scaffold.)
    // `project_text` rejects symlinks that escape the project, so it is the
    // only reader used here: `Path::exists()` follows a symlink and would let a
    // planted `pyproject.toml` outside the tree select a runtime for this project.
    // (`project_text` từ chối symlink trỏ ra ngoài, nên chỉ dùng nó:
    // `Path::exists()` theo symlink và sẽ để `pyproject.toml` cắm từ ngoài chọn
    // runtime cho project này.)
    if runtimes.is_empty()
        && (project_text(project_root, "pyproject.toml").is_some()
            || project_text(project_root, "setup.py").is_some())
    {
        runtimes.push(DetectedRuntime::PythonLib);
    }

    runtimes
}
