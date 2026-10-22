//! Applies shared launcher safety and maps script tokens to optimizer runtime types.
//! Áp dụng launcher safety dùng chung và map token script sang runtime optimizer.

use anyhow::Result;

/// Validate runtime args for Bun/Deno using shared launcher policy.
/// Kiểm tra args runtime cho Bun/Deno dùng policy launcher chung.
///
/// SAFETY: Uses centralized launcher_policy module for consistent validation.
/// AN TOÀN: Dùng module launcher_policy tập trung để kiểm tra nhất quán.
pub(in crate::commands::core::web::implementation) fn validate_runtime_args(
    runtime: &str,
    args: &[&str],
) -> Result<()> {
    use crate::commands::launcher_policy::{LauncherPolicy, Runtime};

    let rt = match runtime {
        "bun" => Runtime::Bun,
        "deno" => Runtime::Deno,
        "node" => Runtime::Node,
        _ => return Ok(()), // Unknown runtime, skip validation
    };

    let policy = LauncherPolicy::dev_server(rt);
    policy.validate_args(args)
}

/// Detect runtime from script tokens for optimizer env loading
/// Phát hiện runtime từ script tokens để load env optimizer
pub(in crate::commands::core::web::implementation) fn detect_runtime_from_tokens(
    tokens: &[&str],
) -> crate::commands::optimizer::runtime_detect::DetectedRuntime {
    use crate::commands::optimizer::runtime_detect::{DetectedRuntime, PackageManager};

    if tokens.is_empty() {
        return DetectedRuntime::Unknown;
    }

    match tokens[0] {
        "bun" => DetectedRuntime::Bun,
        "deno" => DetectedRuntime::Deno,
        "node" | "npm" | "pnpm" | "yarn" | "vite" | "next" | "webpack" | "react-scripts" => {
            DetectedRuntime::NodeJs {
                package_manager: PackageManager::Npm, // Default, actual PM doesn't matter for env loading
            }
        }
        _ => DetectedRuntime::Unknown,
    }
}
