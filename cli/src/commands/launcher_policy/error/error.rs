//! Typed failures with stable user-facing diagnostics.
//! Lỗi có kiểu với diagnostic ổn định hiển thị cho người dùng.

use std::fmt::{self, Display, Formatter};

#[derive(Debug, PartialEq, Eq)]
pub(in crate::commands::launcher_policy) enum LauncherPolicyError {
    DangerousFlag {
        runtime: &'static str,
        flag: &'static str,
    },
    DangerousPermission {
        runtime: &'static str,
        permission: &'static str,
    },
}

impl Display for LauncherPolicyError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::DangerousFlag { runtime, flag } => write!(
                formatter,
                "{}",
                crate::error::runtime_dangerous_flag_rejected(runtime, flag)
            ),
            Self::DangerousPermission {
                runtime,
                permission,
            } => write!(
                formatter,
                "{}",
                crate::error::runtime_dangerous_permission_rejected(runtime, permission)
            ),
        }
    }
}

impl std::error::Error for LauncherPolicyError {}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
