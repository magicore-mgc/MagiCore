//! Typed errors owned by the NuGet native installer.
//! Lỗi typed do installer NuGet native sở hữu.

use mgc_types::MgError;

#[derive(Debug, thiserror::Error)]
pub(in crate::install) enum NugetNativeInstallError {
    #[error(transparent)]
    Shared(#[from] MgError),
    #[error("{0}")]
    Store(#[from] mgc_store::cas::StoreError),
}

impl From<NugetNativeInstallError> for MgError {
    fn from(error: NugetNativeInstallError) -> Self {
        match error {
            NugetNativeInstallError::Shared(error) => error,
            NugetNativeInstallError::Store(error) => Self::Store(error.to_string()),
        }
    }
}
