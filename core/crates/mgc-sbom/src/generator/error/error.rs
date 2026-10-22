//! Generator errors mapped to the stable public SBOM contract — Lỗi generator ánh xạ sang contract SBOM public

/// Errors owned by graph projection, identity resolution, and JSON serialization.
/// Lỗi do phép chiếu graph, phân giải identity và tuần tự hóa JSON sở hữu.
#[derive(Debug, thiserror::Error)]
pub(crate) enum GeneratorError {
    #[error("Invalid lockfile: {0}")]
    InvalidLockfile(String),

    #[error(transparent)]
    Lockfile(#[from] mgc_lockfile::LockfileError),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub(crate) type GeneratorResult<T> = Result<T, GeneratorError>;

impl From<GeneratorError> for crate::SbomError {
    fn from(error: GeneratorError) -> Self {
        match error {
            GeneratorError::InvalidLockfile(message) => Self::InvalidLockfile(message),
            GeneratorError::Lockfile(source) => Self::Lockfile(source),
            GeneratorError::Serialization(source) => Self::SerializationError(source),
        }
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
