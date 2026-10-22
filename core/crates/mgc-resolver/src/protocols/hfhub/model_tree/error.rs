//! Typed local errors for model tree entries.
//! Lỗi cục bộ có type cho entry model tree.

use std::error::Error;
use std::fmt;

use mgc_types::MgError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum ModelTreeError {
    MissingPath,
    EscapesRepo(String),
    MissingBlobOid(String),
    MissingLfsOid(String),
    InvalidLfsSha256(String),
}

impl fmt::Display for ModelTreeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingPath => {
                formatter.write_str("model tree file entry without path (fail-closed)")
            }
            Self::EscapesRepo(path) => {
                write!(
                    formatter,
                    "model tree path escapes the repo: '{path}' (fail-closed)"
                )
            }
            Self::MissingBlobOid(path) => write!(
                formatter,
                "model tree file '{path}' has no hex40 blob oid (fail-closed)"
            ),
            Self::MissingLfsOid(path) => {
                write!(formatter, "LFS file '{path}' without oid (fail-closed)")
            }
            Self::InvalidLfsSha256(path) => {
                write!(
                    formatter,
                    "LFS oid for '{path}' is not a sha256 (fail-closed)"
                )
            }
        }
    }
}

impl Error for ModelTreeError {}

impl From<ModelTreeError> for MgError {
    fn from(error: ModelTreeError) -> Self {
        MgError::Other(error.to_string())
    }
}
