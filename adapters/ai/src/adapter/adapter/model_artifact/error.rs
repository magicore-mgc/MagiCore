//! Local model-artifact failures and the shared MgError boundary.
//! Lỗi cục bộ model-artifact và ranh giới chuyển sang MgError dùng chung.

use mgc_types::MgError;

#[derive(Debug, thiserror::Error)]
pub(super) enum ModelArtifactError {
    #[error("invalid model label: {0}")]
    InvalidLabel(String),
    #[error("model scanner requires async execution")]
    AsyncExecutionRequired,
}

impl From<ModelArtifactError> for MgError {
    fn from(error: ModelArtifactError) -> Self {
        Self::Other(error.to_string())
    }
}
