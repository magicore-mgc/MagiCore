//! Typed cache-size errors — phân biệt sai root và lỗi đo size có source.

use crate::cache::filesystem::logical_size::LogicalSizeError;
use mgc_types::MgError;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub(in crate::cache::cache_size) enum CacheSizeError {
    #[error("AI cache root is not a directory")]
    RootNotDirectory,
    #[error("failed to {operation} AI cache root")]
    Io {
        operation: &'static str,
        #[source]
        source: io::Error,
    },
    #[error(transparent)]
    Filesystem(#[from] LogicalSizeError),
}

impl CacheSizeError {
    pub(in crate::cache::cache_size) fn io(operation: &'static str, source: io::Error) -> Self {
        Self::Io { operation, source }
    }
}

impl From<CacheSizeError> for MgError {
    fn from(error: CacheSizeError) -> Self {
        Self::Anyhow(anyhow::Error::new(error))
    }
}
