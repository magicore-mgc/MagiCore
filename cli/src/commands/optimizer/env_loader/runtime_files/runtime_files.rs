//! Runtime-to-file mapping for optimizer settings.
//! (Ánh xạ runtime sang file cấu hình optimizer.)

use crate::commands::optimizer::runtime_detect::DetectedRuntime;

/// Get the optimizer env files for a detected runtime.
/// Lấy danh sách file env optimizer cho runtime đã phát hiện.
pub(in crate::commands::optimizer::env_loader) fn get_env_files_for_runtime(
    runtime: &DetectedRuntime,
) -> Vec<(&'static str, &'static str)> {
    match runtime {
        // Web runtimes — runtime web.
        // (Các runtime dành cho hệ sinh thái web.)
        DetectedRuntime::Bun => vec![("bun_env.env", "Bun")],
        DetectedRuntime::Deno => vec![("deno_env.env", "Deno")],
        DetectedRuntime::NodeJs { .. } => vec![("node_env.env", "Node.js")],

        // AI runtimes — runtime AI.
        // (Các runtime dành cho hệ sinh thái AI.)
        DetectedRuntime::PythonPyTorch => vec![
            ("pytorch_runtime.env", "PyTorch"),
            ("pytorch_docker.env", "PyTorch Docker"),
        ],
        DetectedRuntime::RustCandle => vec![
            ("candle_runtime.env", "Candle"),
            ("candle_cargo.toml", "Candle Cargo"),
        ],
        DetectedRuntime::GoTensorFlow => {
            vec![("go_ai_runtime.env", "Go AI"), ("go_build.env", "Go Build")]
        }

        // Library runtimes — runtime thư viện.
        DetectedRuntime::RustLib => vec![("rust_cargo_profile.toml", "Rust Lib")],
        DetectedRuntime::GoLib => vec![("go_lib_runtime.env", "Go Lib")],
        DetectedRuntime::PythonLib => vec![("python_lib_runtime.env", "Python Lib")],
        DetectedRuntime::TypeScriptLib => vec![("typescript_lib_env.env", "TypeScript Lib")],

        // App runtimes — runtime ứng dụng.
        DetectedRuntime::Flutter => vec![("flutter_env.env", "Flutter")],
        DetectedRuntime::ReactNative => vec![("react_native_env.env", "React Native")],
        DetectedRuntime::RustNative => vec![("rust_cargo_profile.toml", "Rust Native")],

        // Fallback — dự phòng.
        DetectedRuntime::Unknown => vec![],
    }
}
