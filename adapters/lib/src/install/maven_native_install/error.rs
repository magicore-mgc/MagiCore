//! Typed errors owned by the Maven native installer.
//! Lỗi typed do installer Maven native sở hữu.

use mgc_types::MgError;

#[derive(Debug, thiserror::Error)]
pub(in crate::install) enum MavenNativeInstallError {
    #[error(transparent)]
    Shared(#[from] MgError),
    #[error("{0}")]
    Store(#[from] mgc_store::cas::StoreError),
}

impl From<MavenNativeInstallError> for MgError {
    fn from(error: MavenNativeInstallError) -> Self {
        match error {
            MavenNativeInstallError::Shared(error) => error,
            MavenNativeInstallError::Store(error) => Self::Store(error.to_string()),
        }
    }
}
