//! Shared runtime result types for core detection owners.

/// Detected runtime environment for a project — môi trường runtime đã phát hiện cho project
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DetectedRuntime {
    // Web runtimes — runtime web
    NodeJs { package_manager: PackageManager },
    Deno,
    Bun,

    // AI runtimes — runtime AI
    PythonPyTorch,
    RustCandle,
    GoTensorFlow,

    // Lib runtimes — runtime thư viện
    RustLib,
    GoLib,
    PythonLib,
    TypeScriptLib,

    // App runtimes — runtime ứng dụng
    Flutter,
    ReactNative,
    RustNative,

    // Fallback — dự phòng
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackageManager {
    Npm,
    Pnpm,
    Yarn,
    Bun,
    Deno,
}
