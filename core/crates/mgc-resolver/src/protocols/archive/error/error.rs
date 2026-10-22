//! Typed errors owned by safe archive operations.
//! Lỗi có kiểu thuộc các thao tác archive an toàn.

use mgc_types::MgError;
use std::fmt;
use std::io;

#[derive(Debug)]
pub(in crate::protocols::archive) enum ArchiveError {
    Integrity(String),
    Other(String),
    Io(io::Error),
}

pub(in crate::protocols::archive) type ArchiveResult<T> = Result<T, ArchiveError>;

impl fmt::Display for ArchiveError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integrity(message) | Self::Other(message) => formatter.write_str(message),
            Self::Io(error) => fmt::Display::fmt(error, formatter),
        }
    }
}

impl std::error::Error for ArchiveError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Integrity(_) | Self::Other(_) => None,
        }
    }
}

impl From<io::Error> for ArchiveError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<ArchiveError> for MgError {
    fn from(error: ArchiveError) -> Self {
        match error {
            ArchiveError::Integrity(message) => Self::Integrity(message),
            ArchiveError::Other(message) => Self::Other(message),
            ArchiveError::Io(error) => Self::Io(error),
        }
    }
}
