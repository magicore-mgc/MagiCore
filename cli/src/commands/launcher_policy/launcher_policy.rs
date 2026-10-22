//! Public policy configuration and compatibility facade.
//! Cấu hình policy công khai và facade tương thích.

use anyhow::Result;

use super::{Runtime, argument_validation};

/// Launcher policy configuration.
/// Cấu hình policy launcher.
#[derive(Debug, Clone)]
pub struct LauncherPolicy {
    /// Runtime type being validated.
    /// Loại runtime đang được kiểm tra.
    pub runtime: Runtime,

    /// Whether dangerous permissions are allowed (default: false).
    /// Có cho phép quyền nguy hiểm hay không (mặc định: false).
    pub allow_dangerous_permissions: bool,
}

impl LauncherPolicy {
    /// Create policy for the `mgc dev` launcher.
    /// Tạo policy cho launcher `mgc dev`.
    pub fn dev_server(runtime: Runtime) -> Self {
        Self {
            runtime,
            allow_dangerous_permissions: false,
        }
    }

    /// Create policy for the `mgc test` runner.
    /// Tạo policy cho runner `mgc test`.
    pub fn test_runner(runtime: Runtime) -> Self {
        Self {
            runtime,
            allow_dangerous_permissions: false,
        }
    }

    /// Validate runtime arguments against this policy.
    /// Kiểm tra arguments runtime theo policy này.
    ///
    /// # Errors
    /// Returns an error when an always-blocked flag or a disallowed permission is found.
    /// Trả lỗi khi phát hiện flag luôn bị chặn hoặc permission không được cho phép.
    pub fn validate_args(&self, args: &[&str]) -> Result<()> {
        argument_validation::validate_args(self, args)
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
