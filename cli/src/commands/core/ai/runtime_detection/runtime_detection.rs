//! AI framework-to-optimizer-runtime classification.
//! Phân loại framework AI sang optimizer runtime.

use std::path::Path;

/// Select an optimizer runtime from the AI framework and declared packages.
/// Chọn optimizer runtime theo framework AI và package khai báo.
pub(crate) fn detect_ai_runtime(
    root: &Path,
    framework: &str,
) -> crate::commands::optimizer::runtime_detect::DetectedRuntime {
    use crate::commands::optimizer::runtime_detect::DetectedRuntime;

    match framework {
        "python-agent" | "mcp-server" => {
            // Do not assume PyTorch for generic Python AI frameworks.
            // Không mặc định PyTorch cho framework Python AI chung.
            if crate::commands::optimizer::runtime_detect::python_project_declares_package(
                root, "torch",
            ) {
                DetectedRuntime::PythonPyTorch
            } else {
                DetectedRuntime::Unknown
            }
        }
        "pytorch" | "PyTorch" => DetectedRuntime::PythonPyTorch,
        "candle" | "Candle" => DetectedRuntime::RustCandle,
        "tensorflow-go" | "TensorFlow" => DetectedRuntime::GoTensorFlow,
        _ => DetectedRuntime::Unknown,
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
