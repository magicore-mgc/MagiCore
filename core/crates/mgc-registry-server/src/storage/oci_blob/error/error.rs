//! Typed OCI blob failures with a compatibility-preserving anyhow boundary.
//! Lỗi typed OCI blob với biên anyhow giữ tương thích.

use std::error::Error as StdError;
use std::fmt;

pub(in crate::storage::oci_blob) type OciBlobResult<T> = std::result::Result<T, OciBlobError>;

#[derive(Debug)]
pub(in crate::storage::oci_blob) enum OciBlobError {
    Source(anyhow::Error),
    InvalidRepository(anyhow::Error),
    InvalidDigest(anyhow::Error),
    DigestMismatch {
        declared: String,
        actual: String,
    },
    SizeLimitExceeded,
    IntegrityMismatch {
        digest: String,
        repo: String,
        actual: String,
    },
    Database(sqlx::Error),
    Filesystem(std::io::Error),
}

impl OciBlobError {
    pub(in crate::storage::oci_blob) fn into_anyhow(self) -> anyhow::Error {
        match self {
            Self::Source(error) | Self::InvalidRepository(error) | Self::InvalidDigest(error) => {
                error
            }
            Self::Database(error) => anyhow::Error::new(error),
            Self::Filesystem(error) => anyhow::Error::new(error),
            domain_error => anyhow::Error::msg(domain_error.to_string()),
        }
    }
}

impl fmt::Display for OciBlobError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Source(error) | Self::InvalidRepository(error) | Self::InvalidDigest(error) => {
                write!(formatter, "{error}")
            }
            Self::DigestMismatch { declared, actual } => write!(
                formatter,
                concat!(
                    "oci digest mismatch: declared '{declared}' ",
                    "but bytes hash to '{actual}'"
                ),
                declared = declared,
                actual = actual
            ),
            Self::SizeLimitExceeded => {
                formatter.write_str("OCI blob exceeds configured maximum size")
            }
            Self::IntegrityMismatch {
                digest,
                repo,
                actual,
            } => write!(
                formatter,
                concat!(
                    "oci blob integrity mismatch for '{digest}' in repo '{repo}': ",
                    "bytes hash to '{actual}'"
                ),
                digest = digest,
                repo = repo,
                actual = actual
            ),
            Self::Database(error) => write!(formatter, "{error}"),
            Self::Filesystem(error) => write!(formatter, "{error}"),
        }
    }
}

impl StdError for OciBlobError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Source(error) | Self::InvalidRepository(error) | Self::InvalidDigest(error) => {
                Some(error.as_ref())
            }
            Self::Database(error) => Some(error),
            Self::Filesystem(error) => Some(error),
            Self::DigestMismatch { .. }
            | Self::SizeLimitExceeded
            | Self::IntegrityMismatch { .. } => None,
        }
    }
}

impl From<anyhow::Error> for OciBlobError {
    fn from(error: anyhow::Error) -> Self {
        Self::Source(error)
    }
}

impl From<sqlx::Error> for OciBlobError {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error)
    }
}

impl From<std::io::Error> for OciBlobError {
    fn from(error: std::io::Error) -> Self {
        Self::Filesystem(error)
    }
}
