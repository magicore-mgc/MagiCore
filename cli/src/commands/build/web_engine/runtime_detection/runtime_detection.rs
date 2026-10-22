use std::path::Path;

/// Detect lib runtime for optimizer env loading
/// Phát hiện runtime thư viện để load env optimizer
pub(crate) fn detect_lib_runtime(
    root: &Path,
) -> crate::commands::optimizer::runtime_detect::DetectedRuntime {
    use crate::commands::optimizer::runtime_detect::DetectedRuntime;

    if root.join("Cargo.toml").exists() {
        DetectedRuntime::RustLib
    } else if root.join("pyproject.toml").exists() {
        DetectedRuntime::PythonLib
    } else if root.join("go.mod").exists() {
        DetectedRuntime::GoLib
    } else if root.join("package.json").exists() {
        DetectedRuntime::TypeScriptLib
    } else {
        DetectedRuntime::Unknown
    }
}

/// Detect app runtime for optimizer env loading
/// Phát hiện runtime ứng dụng để load env optimizer
pub(crate) fn detect_app_runtime(
    root: &Path,
) -> crate::commands::optimizer::runtime_detect::DetectedRuntime {
    use crate::commands::optimizer::runtime_detect::DetectedRuntime;
    crate::commands::optimizer::runtime_detect::detect_runtimes(root, "app")
        .first()
        .cloned()
        .unwrap_or(DetectedRuntime::Unknown)
}
