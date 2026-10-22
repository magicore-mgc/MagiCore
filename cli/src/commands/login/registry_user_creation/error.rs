//! Typed failures for registry user creation.
//! Lỗi typed của thao tác tạo user registry.

#[derive(Debug, thiserror::Error)]
pub(in crate::commands::login) enum RegistryUserCreationError {
    #[error(
        "registry authentication requires a valid HTTP(S) URL without credentials, query, or fragment"
    )]
    InvalidUrl,
    #[error("registry authentication requires HTTPS except for loopback registry URLs")]
    RequiresHttps,
    #[error("registry authentication request failed or exceeded its timeout")]
    RequestFailed,
    #[error("registry authentication failed with HTTP {0}")]
    StatusFailed(u16),
    #[error("registry authentication response exceeds the size limit")]
    ResponseTooLarge,
    #[error("registry authentication response is not valid JSON")]
    ResponseInvalid,
    #[error("registry response did not contain a usable token")]
    TokenMissing,
}

pub(in crate::commands::login) fn into_diagnostic(
    error: RegistryUserCreationError,
) -> anyhow::Error {
    match error {
        RegistryUserCreationError::InvalidUrl => crate::error::registry_auth_url_invalid(),
        RegistryUserCreationError::RequiresHttps => crate::error::registry_auth_requires_https(),
        RegistryUserCreationError::RequestFailed => crate::error::registry_auth_request_failed(),
        RegistryUserCreationError::StatusFailed(status) => {
            crate::error::registry_auth_status_failed(status)
        }
        RegistryUserCreationError::ResponseTooLarge => {
            crate::error::registry_auth_response_too_large()
        }
        RegistryUserCreationError::ResponseInvalid => {
            crate::error::registry_auth_response_invalid()
        }
        RegistryUserCreationError::TokenMissing => crate::error::no_token_in_response(),
    }
}
