//! Typed extraction concurrency and blocking-task failures.
//! Lỗi có kiểu cho concurrency extract và blocking task.

use mgc_types::MgError;

#[derive(Debug, thiserror::Error)]
pub(in crate::install::download) enum PipelineError {
    #[error("extract semaphore closed: {0}")]
    ExtractSemaphoreClosed(#[source] tokio::sync::AcquireError),
    #[error("extract task panicked: {0}")]
    ExtractionTaskPanicked(#[source] tokio::task::JoinError),
}

impl From<PipelineError> for MgError {
    fn from(error: PipelineError) -> Self {
        Self::Other(error.to_string())
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
