//! Typed failure for the registry graph's bounded request semaphore.
//! Lỗi có kiểu cho semaphore giới hạn request của graph registry.

use mgc_types::MgError;

#[derive(Debug, thiserror::Error)]
pub(super) enum GraphResolutionError {
    #[error("registry graph resolver semaphore closed")]
    SemaphoreClosed,
}

impl From<GraphResolutionError> for MgError {
    fn from(error: GraphResolutionError) -> Self {
        Self::Other(error.to_string())
    }
}
