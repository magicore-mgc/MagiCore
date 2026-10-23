//! Registry HTTP client construction and proxy-bypass policy.
//! Dựng HTTP client cho registry và policy bỏ qua proxy.

use std::time::Duration;

use reqwest::redirect::Policy;
use url::Url;

use crate::commands::registry::registry_url::error::RegistryUrlError;
use crate::commands::registry::registry_url::is_loopback_host;

const DEFAULT_ADMIN_TIMEOUT_SECS: u64 = 15;
const MAX_ADMIN_TIMEOUT_SECS: u64 = 300;
const ADMIN_TIMEOUT_ENV: &str = "MGC_REGISTRY_ADMIN_TIMEOUT_SECS";

pub(crate) fn client_for_url(
    url: &Url,
    timeout: Duration,
) -> Result<reqwest::Client, RegistryUrlError> {
    let builder = reqwest::Client::builder()
        .redirect(Policy::none())
        .timeout(timeout);
    // Local admin traffic must not escape through ambient HTTP(S)_PROXY settings.
    // Request admin cục bộ không được đi qua HTTP(S)_PROXY do môi trường cấu hình.
    let builder = if should_bypass_system_proxy(url) {
        builder.no_proxy()
    } else {
        builder
    };
    builder.build().map_err(RegistryUrlError::ClientBuild)
}

pub(crate) fn should_bypass_system_proxy(url: &Url) -> bool {
    url.host_str().is_some_and(is_loopback_host)
}

pub(super) fn admin_timeout() -> Result<Duration, RegistryUrlError> {
    let Some(value) = std::env::var_os(ADMIN_TIMEOUT_ENV) else {
        return Ok(Duration::from_secs(DEFAULT_ADMIN_TIMEOUT_SECS));
    };
    let seconds = value
        .to_str()
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|seconds| (1..=MAX_ADMIN_TIMEOUT_SECS).contains(seconds))
        .ok_or(RegistryUrlError::InvalidAdminTimeout)?;
    Ok(Duration::from_secs(seconds))
}

/// Build the bounded admin client using the configured timeout.
/// Tạo client admin với timeout có giới hạn và đọc từ cấu hình.
pub(crate) fn admin_client(url: &Url) -> Result<reqwest::Client, RegistryUrlError> {
    client_for_url(url, admin_timeout()?)
}
