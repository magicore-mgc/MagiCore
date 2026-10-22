//! Typed, URL-redacting Web registry configuration failures.
//! Lỗi cấu hình Web registry typed, không làm lộ URL.

use crate::commands::registry::registry_url::RegistryUrlError;

#[derive(Debug, thiserror::Error)]
pub(in crate::commands::web_registry_config) enum WebRegistryConfigError {
    #[error("MAGICORE_WEB_REGISTRY_URL must contain a valid UTF-8 URL")]
    InvalidEnvironmentValue,
    #[error("invalid Web registry URL or unsupported URL components")]
    InvalidRegistry(#[source] RegistryUrlError),
    #[error("registry URL must use HTTPS except for loopback hosts")]
    RequiresHttps,
}
