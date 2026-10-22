//! Typed errors for login orchestration.
//! Lỗi typed tại ranh giới điều phối đăng nhập.

use super::credentials::CredentialInputError;
use super::registry_user_creation::RegistryUserCreationError;
use super::token_storage::TokenStorageError;

#[derive(Debug, thiserror::Error)]
pub(super) enum LoginError {
    #[error("username is required")]
    UsernameRequired,
    #[error(transparent)]
    Credentials(#[from] CredentialInputError),
    #[error(transparent)]
    Registration(#[from] RegistryUserCreationError),
    #[error(transparent)]
    TokenStorage(#[from] TokenStorageError),
    #[error(transparent)]
    CurrentDirectory(#[from] std::io::Error),
    #[error(transparent)]
    Source(#[from] anyhow::Error),
}

impl LoginError {
    pub(super) fn into_diagnostic(self) -> anyhow::Error {
        match self {
            Self::UsernameRequired => crate::error::registry_auth_username_required(),
            Self::Credentials(source) => super::credentials::credential_input_diagnostic(source),
            Self::Registration(source) => {
                super::registry_user_creation::registration_diagnostic(source)
            }
            Self::TokenStorage(source) => super::token_storage::token_storage_diagnostic(source),
            Self::CurrentDirectory(source) => anyhow::Error::new(source),
            Self::Source(source) => source,
        }
    }
}

#[cfg(test)]
#[path = "test/error.rs"]
mod tests;
