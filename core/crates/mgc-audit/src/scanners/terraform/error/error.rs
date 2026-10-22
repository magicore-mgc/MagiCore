//! Local Terraform lock parser error categories.
//! Các nhóm lỗi parser Terraform lock cục bộ.

use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TerraformLockError {
    MalformedProviderHeader { header: String },
}

impl fmt::Display for TerraformLockError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MalformedProviderHeader { header } => write!(
                formatter,
                "malformed provider header in .terraform.lock.hcl: {header}"
            ),
        }
    }
}

impl std::error::Error for TerraformLockError {}

#[cfg(test)]
#[path = "test/error.rs"]
mod tests;
