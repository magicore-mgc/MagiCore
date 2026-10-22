//! Typed sparse-index parsing and selection failures.
//! Lỗi parse và chọn entry sparse-index có kiểu.

use super::super::http::HttpError;
use mgc_types::MgError;

#[derive(Debug, thiserror::Error)]
pub(in crate::protocols::crates) enum IndexError {
    #[error(transparent)]
    Http(#[from] HttpError),
    #[error("parse crate index entry failed: {0}")]
    ParseEntry(#[source] serde_json::Error),
    #[error("no non-yanked version of {name} matches constraints [{ranges}]")]
    NoMatchingVersion { name: String, ranges: String },
    #[error(
        "crates.io index entry for {name}@{version} has no valid cksum; refusing to resolve an unverifiable artifact"
    )]
    InvalidChecksum { name: String, version: String },
}

pub(in crate::protocols::crates) type IndexResult<T> = Result<T, IndexError>;

impl From<IndexError> for MgError {
    fn from(error: IndexError) -> Self {
        match error {
            IndexError::Http(error) => error.into(),
            IndexError::ParseEntry(error) => {
                Self::Other(format!("parse crate index entry failed: {error}"))
            }
            IndexError::NoMatchingVersion { name, ranges } => Self::Other(format!(
                "no non-yanked version of {name} matches constraints [{ranges}]"
            )),
            IndexError::InvalidChecksum { name, version } => Self::Integrity(format!(
                "crates.io index entry for {name}@{version} has no valid cksum; refusing to resolve an unverifiable artifact"
            )),
        }
    }
}
