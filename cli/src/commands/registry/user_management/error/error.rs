//! Typed failures for registry user administration.
//! Lỗi typed của quản trị user registry.

use super::super::super::registry_url::RegistryUrlError;

#[derive(Debug, thiserror::Error)]
pub(in crate::commands::registry) enum UserManagementError {
    #[error("registry user password input failed: {0}")]
    PasswordInput(#[source] anyhow::Error),
    #[error("registry user creation failed: {0}")]
    UserCreation(#[source] anyhow::Error),
    #[error("cannot resolve the registry authentication config path: {0}")]
    ConfigHome(#[source] anyhow::Error),
    #[error("cannot save the registry authentication token: {0}")]
    TokenPersistence(#[source] anyhow::Error),
    #[error("Admin token required (--admin-token or MAGICORE_REGISTRY_ADMIN_TOKEN)")]
    AdminTokenRequired,
    #[error("admin token must be a non-empty printable value")]
    InvalidAdminToken,
    #[error("registry token must not be empty")]
    EmptyToken,
    #[error(transparent)]
    RegistryUrl(#[from] RegistryUrlError),
    #[error("registry request failed")]
    RequestFailed(#[source] reqwest::Error),
    #[error("User {0} not found")]
    UserNotFound(String),
    #[error("Delete user failed: HTTP {0}")]
    DeleteFailed(u16),
    #[error("Token not found (was it revoked?)")]
    TokenNotFound,
    #[error("Revoke token failed: HTTP {0}")]
    RevokeFailed(u16),
}

pub(in crate::commands::registry) type UserManagementResult<T> = Result<T, UserManagementError>;
