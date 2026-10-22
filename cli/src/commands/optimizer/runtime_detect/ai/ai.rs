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

    runtimes
}
