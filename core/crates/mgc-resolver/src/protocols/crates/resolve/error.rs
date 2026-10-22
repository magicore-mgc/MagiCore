//! Typed single-package Crates.io resolution failures.
//! Lỗi resolve một package Crates.io có kiểu.

use super::super::http::HttpError;
use mgc_types::MgError;

#[derive(Debug, thiserror::Error)]
pub(in crate::protocols::crates) enum ResolveError {
    #[error(transparent)]
    Http(#[from] HttpError),
    #[error("parse crate index entry failed: {0}")]
    ParseEntry(#[source] serde_json::Error),
    #[error("no non-yanked version of {name} matches range '{range}'")]
    NoMatchingVersion { name: String, range: String },
    #[error(
        "crates.io index entry for {name}@{version} has no valid cksum; refusing to resolve an unverifiable artifact"
    )]
    InvalidChecksum { name: String, version: String },
    #[error(
        "crates.io package '{package}'@{version} requires dependency '{dependency}' from alternate registry '{registry}', which MagiCore cannot resolve yet"
    )]
    AlternateRegistry {
        package: String,
        version: String,
        dependency: String,
        registry: String,
    },
}

pub(in crate::protocols::crates) type ResolveResult<T> = Result<T, ResolveError>;

impl From<ResolveError> for MgError {
    fn from(error: ResolveError) -> Self {
        match error {
            ResolveError::Http(error) => error.into(),
            ResolveError::ParseEntry(error) => {
                Self::Other(format!("parse crate index entry failed: {error}"))
            }
            ResolveError::NoMatchingVersion { name, range } => Self::Other(format!(
                "no non-yanked version of {name} matches range '{range}'"
            )),
            ResolveError::InvalidChecksum { name, version } => Self::Integrity(format!(
                "crates.io index entry for {name}@{version} has no valid cksum; refusing to resolve an unverifiable artifact"
            )),
            ResolveError::AlternateRegistry {
                package,
                version,
                dependency,
                registry,
            } => Self::Other(format!(
                "crates.io package '{package}'@{version} requires dependency '{dependency}' from alternate registry '{registry}', which MagiCore cannot resolve yet"
            )),
        }
    }
}
