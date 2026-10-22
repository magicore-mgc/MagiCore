//! Typed failures for requested-package metadata validation.
//! Lỗi có kiểu khi kiểm metadata của package được yêu cầu.

use anyhow::Error as AnyhowError;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub(crate) enum InstalledMetadataError {
    InvalidPackageMetadata,
}

impl Display for InstalledMetadataError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidPackageMetadata => write!(
                formatter,
                "package runner: installed package identity or executable metadata is invalid"
            ),
        }
    }
}

impl std::error::Error for InstalledMetadataError {}

pub fn registry_package_invalid_bins() -> AnyhowError {
    AnyhowError::new(InstalledMetadataError::InvalidPackageMetadata)
}

#[cfg(test)]
#[path = "test/error.rs"]
mod tests;
