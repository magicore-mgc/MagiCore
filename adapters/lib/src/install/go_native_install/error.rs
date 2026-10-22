//! Typed errors owned by the Go native installer.
//! Lỗi typed do installer Go native sở hữu.

use mgc_types::MgError;

#[derive(Debug, thiserror::Error)]
pub(in crate::install) enum GoNativeInstallError {
    #[error(transparent)]
    Shared(#[from] MgError),
    #[error("{0}")]
    Store(#[from] mgc_store::cas::StoreError),
}

impl From<GoNativeInstallError> for MgError {
    fn from(error: GoNativeInstallError) -> Self {
        match error {
            GoNativeInstallError::Shared(error) => error,
            GoNativeInstallError::Store(error) => Self::Store(error.to_string()),
        }
    }
}
