//! Typed model-path validation failure — lỗi typed khi model ID có cú pháp path nguy hiểm.

use mgc_types::MgError;
use thiserror::Error;

#[derive(Debug, Error)]
pub(in crate::cache::model_cache_path) enum ModelCachePathError {
    #[error("model identifier is empty or contains an unsafe path segment")]
    InvalidModelId,
}

impl From<ModelCachePathError> for MgError {
    fn from(error: ModelCachePathError) -> Self {
        Self::Anyhow(anyhow::Error::new(error))
    }
}
