//! Typed filesystem failures for staging and atomic tarball publication.
//! Lỗi filesystem có kiểu cho staging và publish tarball nguyên tử.

use mgc_types::MgError;
use std::io;

#[derive(Debug, thiserror::Error)]
pub(crate) enum StagingError {
    #[error("{0}")]
    CreateDirectory(#[source] io::Error),
    #[error("cannot inspect concurrent tarball cache winner: {0}")]
    InspectWinner(#[source] io::Error),
    #[error("concurrent tarball cache winner is not a regular file: '{0}'")]
    WinnerNotRegular(String),
    #[error("cannot verify concurrent cache winner: {0}")]
    OpenWinner(#[source] io::Error),
    #[error("cannot hash concurrent cache winner: {0}")]
    ReadWinner(#[source] io::Error),
    #[error(
        "concurrent tarball cache conflict for '{0}': winner integrity does not match downloaded artifact"
    )]
    WinnerIntegrityConflict(String),
    #[error("cannot atomically publish tarball cache entry '{path}': {source}")]
    Publish {
        path: String,
        #[source]
        source: io::Error,
    },
    #[error("published tarball cache entry but failed to remove staging file '{path}': {source}")]
    RemoveStaging {
        path: String,
        #[source]
        source: io::Error,
    },
}

impl From<StagingError> for MgError {
    fn from(error: StagingError) -> Self {
        Self::Store(error.to_string())
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
