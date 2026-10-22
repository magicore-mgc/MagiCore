//! Typed prefetch task and semaphore failures.
//! Lỗi có kiểu cho task prefetch và semaphore.

use mgc_types::MgError;

#[derive(Debug, thiserror::Error)]
pub(in crate::install::download) enum PrefetchError {
    #[error("download semaphore closed: {0}")]
    SemaphoreClosed(#[source] tokio::sync::AcquireError),
    #[error("download task failed: {0}")]
    JoinTask(#[source] tokio::task::JoinError),
}

impl From<PrefetchError> for MgError {
    fn from(error: PrefetchError) -> Self {
        Self::Other(error.to_string())
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
