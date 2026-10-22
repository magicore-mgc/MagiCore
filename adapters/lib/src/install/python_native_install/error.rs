//! Typed errors owned by the Python native installer.
//! Lỗi typed do installer Python native sở hữu.

use mgc_types::MgError;

#[derive(Debug, thiserror::Error)]
pub(in crate::install) enum PythonNativeInstallError {
    #[error(transparent)]
    Shared(#[from] MgError),
    #[error("{0}")]
    Store(#[from] mgc_store::cas::StoreError),
    #[error(
        "offline Python install cache miss for '{name}'@{version}; run `mgc install` online before using `--offline`"
    )]
    OfflineCacheMiss { name: String, version: String },
    #[error("offline Python install cache entry disappeared for '{name}'@{version}")]
    OfflineCacheDisappeared { name: String, version: String },
    #[error("Python artifact for '{name}'@{version} became non-importable after preflight")]
    ArtifactBecameNonImportable { name: String, version: String },
}

impl From<PythonNativeInstallError> for MgError {
    fn from(error: PythonNativeInstallError) -> Self {
        match error {
            PythonNativeInstallError::Shared(error) => error,
            PythonNativeInstallError::Store(error) => Self::Store(error.to_string()),
            error => Self::Other(error.to_string()),
        }
    }
}
