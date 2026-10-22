//! Typed failures for inherited install-policy loading.
//! Lỗi có kiểu khi đọc policy cài đặt được kế thừa.

use anyhow::Error as AnyhowError;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub(crate) enum InstallPolicyError {
    CallerPolicyInvalid,
}

impl Display for InstallPolicyError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CallerPolicyInvalid => write!(
                formatter,
                "package runner: cannot read caller project policy; fix mgc.toml before running the package"
            ),
        }
    }
}

impl std::error::Error for InstallPolicyError {}

pub fn registry_package_invalid_caller_policy() -> AnyhowError {
    AnyhowError::new(InstallPolicyError::CallerPolicyInvalid)
}

#[cfg(test)]
#[path = "test/error.rs"]
mod tests;
