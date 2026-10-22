//! Typed failures for signed manifest cache operations.
//! Lỗi typed cho thao tác cache manifest đã ký.

use std::io;

#[derive(Debug, thiserror::Error)]
pub(in crate::commands::policy) enum PolicyManifestCacheError {
    #[error("policy manifest exceeds 1 MiB; download was not cached")]
    OversizedManifest,
    #[error("policy manifest is not valid JSON: {0}")]
    InvalidJson(#[source] serde_json::Error),
    #[error("fetched policy manifest does not verify")]
    UnverifiedManifest,
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error("signed policy manifest update rejected: {0}")]
    UpdateRejected(String),
    #[error("cannot lock signed policy manifest cache: cannot open '{path}': {source}")]
    LockOpen {
        path: String,
        #[source]
        source: io::Error,
    },
    #[error("cannot lock signed policy manifest cache: cannot inspect '{path}': {source}")]
    LockPathInspect {
        path: String,
        #[source]
        source: io::Error,
    },
    #[error("cannot lock signed policy manifest cache: cannot inspect opened '{path}': {source}")]
    LockFileInspect {
        path: String,
        #[source]
        source: io::Error,
    },
    #[error("cannot lock signed policy manifest cache: lock path is not a regular file")]
    LockPathNotRegular,
    #[error("cannot lock signed policy manifest cache: cannot acquire '{path}': {source}")]
    LockAcquire {
        path: String,
        #[source]
        source: io::Error,
    },
    #[error("{0}")]
    Serialization(#[source] serde_json::Error),
}

pub(in crate::commands::policy) type PolicyManifestCacheResult<T> =
    std::result::Result<T, PolicyManifestCacheError>;
