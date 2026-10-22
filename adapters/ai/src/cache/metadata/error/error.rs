//! Typed cache metadata failures — lỗi metadata typed và giữ source gốc.

use mgc_platform::OpenRegularFileError;
use mgc_types::MgError;
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub(in crate::cache::metadata) enum MetadataError {
    #[error("failed to {operation} AI cache metadata")]
    Io {
        operation: &'static str,
        #[source]
        source: io::Error,
    },
    #[error("failed to serialize AI cache metadata")]
    Serialize(#[source] serde_json::Error),
    #[error("failed to parse AI cache metadata")]
    Deserialize(#[source] serde_json::Error),
    #[error("AI cache metadata sidecar is a symbolic link")]
    SymlinkSidecar,
    #[error("AI cache metadata sidecar is not a regular file")]
    NonRegularSidecar,
    #[error("AI cache metadata path has no filename")]
    InvalidSidecarPath,
    #[error("failed to open AI cache metadata without following links: {0}")]
    OpenSidecar(#[source] OpenRegularFileError),
}

impl MetadataError {
    pub(in crate::cache::metadata) fn io(operation: &'static str, source: io::Error) -> Self {
        Self::Io { operation, source }
    }
}

impl From<MetadataError> for MgError {
    fn from(error: MetadataError) -> Self {
        Self::Anyhow(anyhow::Error::new(error))
    }
}
