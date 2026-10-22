//! Typed SPDX failures and legacy boundary conversion — Lỗi SPDX typed và ánh xạ boundary cũ

use crate::SbomError;

pub(in crate::spdx) type SpdxResult<T> = Result<T, SpdxError>;

#[derive(Debug, thiserror::Error)]
pub(in crate::spdx) enum SpdxError {
    #[error("SPDX document name must not be empty")]
    EmptyDocumentName,
    #[error("SPDX package name must not be empty")]
    EmptyPackageName,
    #[error("duplicate SPDX package identity: {0}")]
    DuplicatePackageIdentity(String),
    #[error("SPDX package identity disappeared: {0}")]
    PackageIdentityDisappeared(String),
    #[error("SPDX dependency source is missing from package graph: {0}")]
    DependencySourceMissing(String),
    #[error("SPDX dependency target is missing from package graph: {0}")]
    DependencyTargetMissing(String),
    #[error("SPDX serialization failed: {0}")]
    Serialization(#[source] serde_json::Error),
}

impl SpdxError {
    pub(in crate::spdx) fn into_public(self) -> SbomError {
        match self {
            Self::EmptyDocumentName => {
                SbomError::InvalidLockfile("SPDX document name must not be empty".to_string())
            }
            Self::EmptyPackageName => {
                SbomError::InvalidLockfile("SPDX package name must not be empty".to_string())
            }
            Self::DuplicatePackageIdentity(bom_ref) => {
                SbomError::InvalidLockfile(format!("duplicate SPDX package identity: {bom_ref}"))
            }
            Self::PackageIdentityDisappeared(bom_ref) => {
                SbomError::InvalidLockfile(format!("SPDX package identity disappeared: {bom_ref}"))
            }
            Self::DependencySourceMissing(bom_ref) => SbomError::InvalidLockfile(format!(
                "SPDX dependency source is missing from package graph: {bom_ref}"
            )),
            Self::DependencyTargetMissing(bom_ref) => SbomError::InvalidLockfile(format!(
                "SPDX dependency target is missing from package graph: {bom_ref}"
            )),
            Self::Serialization(source) => SbomError::SerializationError(source),
        }
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
