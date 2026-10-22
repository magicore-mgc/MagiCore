//! Graph-resolution failures mapped to the established MgError categories.
//! Lỗi resolve graph được ánh xạ về các nhóm MgError hiện hành.

use mgc_types::MgError;

#[derive(Debug, thiserror::Error)]
pub(in crate::protocols::pubdev) enum GraphResolutionError {
    #[error(transparent)]
    Shared(#[from] MgError),
    #[error("pub.dev resolver semaphore closed")]
    SemaphoreClosed,
    #[error("missing cached pub.dev metadata for '{0}'")]
    MissingCachedMetadata(String),
    #[error(
        "pub.dev dependency selection oscillates across graph passes; refusing to emit an unstable lock graph"
    )]
    OscillatingSelection,
    #[error("pub.dev dependency graph did not converge within {0} passes")]
    DidNotConverge(usize),
}

impl GraphResolutionError {
    pub(in crate::protocols::pubdev) fn into_mg_error(self) -> MgError {
        match self {
            Self::Shared(error) => error,
            Self::SemaphoreClosed => MgError::Other("pub.dev resolver semaphore closed".to_string()),
            Self::MissingCachedMetadata(name) => MgError::Integrity(format!(
                "missing cached pub.dev metadata for '{name}'"
            )),
            Self::OscillatingSelection => MgError::DependencyConflict(
                "pub.dev dependency selection oscillates across graph passes; refusing to emit an unstable lock graph".to_string(),
            ),
            Self::DidNotConverge(rounds) => MgError::DependencyConflict(format!(
                "pub.dev dependency graph did not converge within {rounds} passes"
            )),
        }
    }
}
