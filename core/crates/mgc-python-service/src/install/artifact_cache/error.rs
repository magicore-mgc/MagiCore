//! Local cache I/O and integrity errors for Python artifacts.
//! Lỗi I/O cache và toàn vẹn cục bộ cho artifact Python.

use std::{error::Error, fmt, io};

use mgc_types::MgError;

#[derive(Debug)]
pub(in crate::install) enum PythonArtifactCacheError {
    NotRegularFile {
        path: String,
    },
    InspectPath {
        path: String,
        source: io::Error,
    },
    OpenNoFollow {
        path: String,
        source: io::Error,
    },
    InspectOpenedFile {
        path: String,
        source: io::Error,
    },
    #[cfg(windows)]
    ReparsePoint {
        path: String,
    },
    ReadFile {
        path: String,
        source: io::Error,
    },
    DigestMismatch {
        path: String,
        expected: String,
        actual: String,
    },
}

impl fmt::Display for PythonArtifactCacheError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotRegularFile { path } => {
                write!(
                    formatter,
                    "Python artifact cache entry '{path}' is not a regular file"
                )
            }
            Self::InspectPath { path, source } => {
                write!(
                    formatter,
                    "cannot inspect Python artifact cache '{path}': {source}"
                )
            }
            Self::OpenNoFollow { path, source } => write!(
                formatter,
                "cannot open Python artifact cache '{path}' without following links: {source}"
            ),
            Self::InspectOpenedFile { path, source } => write!(
                formatter,
                "cannot inspect opened Python artifact cache '{path}': {source}"
            ),
            #[cfg(windows)]
            Self::ReparsePoint { path } => {
                write!(
                    formatter,
                    "Python artifact cache entry '{path}' is a reparse point"
                )
            }
            Self::ReadFile { path, source } => {
                write!(
                    formatter,
                    "cannot read Python artifact cache '{path}': {source}"
                )
            }
            Self::DigestMismatch {
                path,
                expected,
                actual,
            } => write!(
                formatter,
                "Python artifact cache integrity mismatch for '{path}': expected SHA-256 {expected}, got {actual}"
            ),
        }
    }
}

impl Error for PythonArtifactCacheError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InspectPath { source, .. }
            | Self::OpenNoFollow { source, .. }
            | Self::InspectOpenedFile { source, .. }
            | Self::ReadFile { source, .. } => Some(source),
            Self::NotRegularFile { .. } | Self::DigestMismatch { .. } => None,
            #[cfg(windows)]
            Self::ReparsePoint { .. } => None,
        }
    }
}

impl From<PythonArtifactCacheError> for MgError {
    fn from(error: PythonArtifactCacheError) -> Self {
        match error {
            error @ PythonArtifactCacheError::InspectPath { .. }
            | error @ PythonArtifactCacheError::ReadFile { .. } => Self::Other(error.to_string()),
            error => Self::Integrity(error.to_string()),
        }
    }
}
