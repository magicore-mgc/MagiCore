//! Typed failures for registry URL and client policy.
//! Lỗi typed cho chính sách URL registry và HTTP client.

#[derive(Debug, thiserror::Error)]
pub(crate) enum RegistryUrlError {
    #[error("registry URL is invalid or contains credentials, query, or fragment")]
    InvalidRegistry,
    #[error("registry URL requires HTTPS except for loopback hosts")]
    RequiresHttps,
    #[error("registry URL cannot be extended with endpoint path segments")]
    InvalidPath,
    #[error("MGC_REGISTRY_ADMIN_TIMEOUT_SECS must be between 1 and 300 seconds")]
    InvalidAdminTimeout,
    #[error("registry HTTP client could not be constructed")]
    ClientBuild(#[source] reqwest::Error),
}
