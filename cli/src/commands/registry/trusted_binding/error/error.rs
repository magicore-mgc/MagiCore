//! Typed failures for trusted publisher binding.
//! Lỗi typed của trusted publisher binding.

#[derive(Debug, thiserror::Error)]
pub(in crate::commands::registry) enum TrustedBindingError {
    #[error("trusted publisher management requires --admin-token or MAGICORE_REGISTRY_ADMIN_TOKEN")]
    AdminTokenRequired,
    #[error("trusted publishing requires a valid registry URL without credentials or fragment")]
    InvalidRegistry,
    #[error("trusted publishing requires HTTPS except for loopback registry URLs")]
    RequiresHttps,
    #[error("trusted publisher management request failed")]
    RequestFailed(#[source] reqwest::Error),
    #[error("trusted publisher binding request failed with HTTP {0}")]
    BindingFailed(u16),
    #[error(transparent)]
    ClientPolicy(#[from] super::super::super::registry_url::RegistryUrlError),
}
