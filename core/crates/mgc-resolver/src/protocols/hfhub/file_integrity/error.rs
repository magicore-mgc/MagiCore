//! Typed local file-integrity errors.
//! Lỗi toàn vẹn file cục bộ có type.

use std::error::Error;
use std::fmt;

use mgc_types::MgError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum FileIntegrityError {
    Sha256Mismatch {
        path: String,
        expected: String,
        actual: String,
    },
    GitBlobMismatch {
        path: String,
        expected: String,
        actual: String,
    },
}

impl fmt::Display for FileIntegrityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sha256Mismatch {
                path,
                expected,
                actual,
            } => write!(
                formatter,
                "sha256 mismatch for '{path}': expected {expected}, got {actual}"
            ),
            Self::GitBlobMismatch {
                path,
                expected,
                actual,
            } => write!(
                formatter,
                "git blob mismatch for '{path}': expected {expected}, got {actual}"
            ),
        }
    }
}

impl Error for FileIntegrityError {}

impl From<FileIntegrityError> for MgError {
    fn from(error: FileIntegrityError) -> Self {
        MgError::Integrity(error.to_string())
    }
}
