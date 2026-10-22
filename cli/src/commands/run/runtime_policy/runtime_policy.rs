//! Detect project runtime and resolve task-script timeout.
//! Phát hiện runtime project và xác định timeout cho task script.

use std::path::Path;
use std::time::Duration;

const DEFAULT_RUN_SCRIPT_TIMEOUT_SECS: u64 = 300;
const RUN_SCRIPT_TIMEOUT_ENV: &str = "MGC_RUN_SCRIPT_TIMEOUT_SECS";

pub(in crate::commands::run) fn run_script_timeout() -> Duration {
    std::env::var(RUN_SCRIPT_TIMEOUT_ENV)
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|secs| *secs > 0)
        .map(Duration::from_secs)
        .unwrap_or(Duration::from_secs(DEFAULT_RUN_SCRIPT_TIMEOUT_SECS))
}

/// Detect runtime for optimizer env loading based on project files
/// Phát hiện runtime để load env optimizer dựa trên file project
pub(in crate::commands::run) fn detect_run_runtime(
    cwd: &Path,
) -> crate::commands::optimizer::runtime_detect::DetectedRuntime {
    use crate::commands::optimizer::runtime_detect::{DetectedRuntime, PackageManager};

    if cwd.join("Cargo.toml").exists() {
        DetectedRuntime::RustLib
    } else if cwd.join("go.mod").exists() {
        DetectedRuntime::GoLib
    } else if cwd.join("pyproject.toml").exists() || cwd.join("setup.py").exists() {
        DetectedRuntime::PythonLib
    } else if cwd.join("pubspec.yaml").exists() {
        DetectedRuntime::Flutter
    } else if cwd.join("package.json").exists() {
        // Detect package manager for web runtime
        if cwd.join("bun.lockb").exists() {
            DetectedRuntime::Bun
        } else if cwd.join("deno.json").exists() || cwd.join("deno.jsonc").exists() {
            DetectedRuntime::Deno
        } else {
            DetectedRuntime::NodeJs {
                package_manager: PackageManager::Npm,
            }
        }
    } else {
        DetectedRuntime::Unknown
    }
}
