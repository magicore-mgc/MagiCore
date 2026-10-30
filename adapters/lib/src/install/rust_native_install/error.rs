//! Typed cache, filesystem, and delegated errors for Rust native install.
//! Lỗi typed cache, filesystem và lỗi delegate của install Rust native.

use mgc_types::MgError;
use std::io;

#[derive(Debug, thiserror::Error)]
pub(crate) enum RustNativeInstallError {
    #[error(transparent)]
    Shared(#[from] MgError),
    #[error("{0}")]
    Store(#[from] mgc_store::cas::StoreError),
    #[error("Rust source cache entry '{path}' is not a regular directory")]
    SourceCacheNotRegularDirectory { path: String },
    #[error(
        "Rust source cache entry '{path}' is not fully materialized (Cargo's readiness marker is missing or invalid); run `mgc install` so the source is rebuilt from its verified archive"
    )]
    SourceNotMaterialized { path: String },
    #[error("cannot write Cargo's registry source readiness marker: {source}")]
    SourceMarker { source: io::Error },
    #[error("cannot inspect Rust source cache '{path}': {source}")]
    SourceCacheInspection { path: String, source: io::Error },
    #[error(
        "offline Rust crate cache miss for '{name}'@{version}; run `mgc install-lib` online before using `--offline`"
    )]
    OfflineCacheMiss { name: String, version: String },
    #[error("offline Rust crate cache entry disappeared for '{name}'@{version} after preflight")]
    OfflineCacheDisappeared { name: String, version: String },
    #[error("Rust crate cache entry '{path}' is not a regular file")]
    CacheNotRegularFile { path: String },
    #[error("cannot inspect Rust crate cache '{path}': {source}")]
    CacheInspection { path: String, source: io::Error },
    #[error("cannot open Rust crate cache '{path}' without following links: {source}")]
    CacheOpen { path: String, source: io::Error },
    #[error("cannot inspect opened Rust crate cache '{path}': {source}")]
    OpenedCacheInspection { path: String, source: io::Error },
    #[error("Rust crate cache entry '{path}' is a reparse point")]
    #[cfg_attr(not(windows), allow(dead_code))]
    CacheReparsePoint { path: String },
    #[error("cannot read Rust crate cache '{path}': {source}")]
    CacheRead { path: String, source: io::Error },
    #[error(
        "Rust crate cache integrity mismatch for '{path}': expected SHA-256 {expected}, got {actual}"
    )]
    CacheIntegrityMismatch {
        path: String,
        expected: String,
        actual: String,
    },
}

impl RustNativeInstallError {
    fn is_integrity_failure(&self) -> bool {
        matches!(
            self,
            Self::SourceCacheNotRegularDirectory { .. }
                | Self::SourceNotMaterialized { .. }
                | Self::CacheNotRegularFile { .. }
                | Self::CacheOpen { .. }
                | Self::OpenedCacheInspection { .. }
                | Self::CacheReparsePoint { .. }
                | Self::CacheIntegrityMismatch { .. }
        )
    }
}

impl From<RustNativeInstallError> for MgError {
    fn from(error: RustNativeInstallError) -> Self {
        match error {
            RustNativeInstallError::Shared(error) => error,
            RustNativeInstallError::Store(error) => Self::Store(error.to_string()),
            error if error.is_integrity_failure() => Self::Integrity(error.to_string()),
            error => Self::Other(error.to_string()),
        }
    }
}
