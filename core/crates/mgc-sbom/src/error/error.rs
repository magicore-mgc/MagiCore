//! Public SBOM errors and result alias — Lỗi public SBOM và alias kết quả

use anyhow::Result;

/// SBOM error — Lỗi SBOM
#[derive(Debug, thiserror::Error)]
pub enum SbomError {
    #[error("Invalid lockfile: {0}")]
    InvalidLockfile(String),

    #[error(transparent)]
    Lockfile(#[from] mgc_lockfile::LockfileError),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type SbomResult<T> = Result<T, SbomError>;

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
