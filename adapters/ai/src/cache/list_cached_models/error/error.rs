//! Typed cache-list errors — giữ operation và source I/O gốc.

use mgc_types::MgError;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub(in crate::cache::list_cached_models) enum ListCachedModelsError {
    #[error("failed to {operation} AI model cache")]
    Io {
        operation: &'static str,
        #[source]
        source: io::Error,
    },
    #[error("AI model cache root is not a directory")]
    CacheRootNotDirectory,
}

impl ListCachedModelsError {
    pub(in crate::cache::list_cached_models) fn io(
        operation: &'static str,
        source: io::Error,
    ) -> Self {
        Self::Io { operation, source }
    }
}

impl From<ListCachedModelsError> for MgError {
    fn from(error: ListCachedModelsError) -> Self {
        Self::Anyhow(anyhow::Error::new(error))
    }
}
