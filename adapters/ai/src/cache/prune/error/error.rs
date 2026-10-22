//! Typed AI cache prune errors — giữ context operation và source I/O gốc.

use crate::cache::filesystem::logical_size::LogicalSizeError;
use mgc_types::MgError;
use std::io;
use thiserror::Error;

pub(in crate::cache::prune) type PruneOutcome<T> = Result<T, PruneError>;

#[derive(Debug, Error)]
pub(in crate::cache::prune) enum PruneError {
    #[error("failed to {operation} AI cache")]
    Io {
        operation: &'static str,
        #[source]
        source: io::Error,
    },
    #[error(transparent)]
    Filesystem(#[from] LogicalSizeError),
    #[error("cache root is not a directory")]
    CacheRootNotDirectory,
    #[error("refusing to remove a symbolic-link model cache entry")]
    SymbolicLinkEntry,
    #[error("refusing to remove an unsupported model cache filesystem entry")]
    UnsupportedEntry,
}

impl PruneError {
    pub(in crate::cache::prune) fn io(operation: &'static str, source: io::Error) -> Self {
        Self::Io { operation, source }
    }
}

impl From<PruneError> for MgError {
    fn from(error: PruneError) -> Self {
        Self::Anyhow(anyhow::Error::new(error))
    }
}
