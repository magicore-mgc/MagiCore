//! Typed failures for package-run caller-directory validation.
//! Lỗi có kiểu khi xác minh working directory của package runner.

use anyhow::Error as AnyhowError;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub(crate) enum RunRegistryPackageError {
    ExecutionCwdNotReal,
}

impl Display for RunRegistryPackageError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExecutionCwdNotReal => write!(
                formatter,
                "package execution working directory must be a real directory"
            ),
        }
    }
}

impl std::error::Error for RunRegistryPackageError {}

pub fn registry_package_execution_cwd_not_real() -> AnyhowError {
    AnyhowError::new(RunRegistryPackageError::ExecutionCwdNotReal)
}

#[cfg(test)]
#[path = "test/error.rs"]
mod tests;
