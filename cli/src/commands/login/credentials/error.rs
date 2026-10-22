//! Typed failures for credential input.
//! Lỗi typed cho bước nhập credential.

#[derive(Debug, thiserror::Error)]
pub(in crate::commands::login) enum CredentialInputError {
    #[error("password input exceeds the size limit")]
    TooLarge,
    #[error("password input was empty")]
    Empty,
    #[error(transparent)]
    Stdin(#[from] std::io::Error),
    #[error(transparent)]
    Prompt(anyhow::Error),
}

pub(in crate::commands::login) fn into_diagnostic(error: CredentialInputError) -> anyhow::Error {
    match error {
        CredentialInputError::TooLarge => crate::error::registry_auth_password_too_large(),
        CredentialInputError::Empty => crate::error::registry_auth_password_required(),
        CredentialInputError::Stdin(source) => anyhow::Error::new(source),
        CredentialInputError::Prompt(source) => source,
    }
}
