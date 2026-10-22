//! Ordered checks for runtime flags and Deno permissions.
//! Kiểm tra có thứ tự cho runtime flags và Deno permissions.

use anyhow::Result;

use super::super::{LauncherPolicy, Runtime, error::LauncherPolicyError, runtime};

pub(in crate::commands::launcher_policy) fn validate_args(
    policy: &LauncherPolicy,
    args: &[&str],
) -> Result<()> {
    for arg in args {
        for flag in runtime::dangerous_flags(policy.runtime) {
            let flag = *flag;
            if arg.starts_with(flag) {
                return Err(LauncherPolicyError::DangerousFlag {
                    runtime: policy.runtime.name(),
                    flag,
                }
                .into());
            }
        }
    }

    if matches!(policy.runtime, Runtime::Deno) && !policy.allow_dangerous_permissions {
        for arg in args {
            for permission in runtime::dangerous_permissions(policy.runtime) {
                let permission = *permission;
                if arg.starts_with(permission) {
                    return Err(LauncherPolicyError::DangerousPermission {
                        runtime: policy.runtime.name(),
                        permission,
                    }
                    .into());
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
