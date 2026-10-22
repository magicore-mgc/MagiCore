//! Fetch npm versions using the configured registry and pooled CLI client.
//! Tải version npm qua registry cấu hình và HTTP client pool dùng chung.

use super::super::{ScaffoldAgePolicy, error as web_errors};
use anyhow::Result;

/// Fetch metadata with the registry adapter's age gate armed.
/// Tải metadata với age gate của registry adapter được bật.
#[cfg(feature = "web")]
pub(in crate::commands::core::web::implementation::versions) async fn fetch_age_gated_npm_metadata(
    package: &str,
) -> Result<mgc_web_adapter::native::npm_registry::PackageMetadata> {
    let registry_url = crate::commands::web_registry_config::web_registry_url()?;
    let registry = mgc_web_adapter::native::npm_registry::NpmRegistry::new(&registry_url);
    registry.set_age_gate_armed(true);
    registry
        .fetch_metadata(package)
        .await
        .map_err(|error| web_errors::web_scaffold_age_metadata_unavailable(package, &error))
}

/// Resolve latest under the same age policy used by installation.
/// Resolve latest theo age policy đang dùng khi install.
#[cfg(feature = "web")]
pub(crate) async fn fetch_npm_latest_version(
    package: &str,
    age_policy: Option<ScaffoldAgePolicy>,
) -> Result<String> {
    if let Some(policy) = age_policy.filter(|policy| policy.cutoff_hours > 0) {
        let metadata = fetch_age_gated_npm_metadata(package).await?;
        return super::super::eligible_release::select_latest_eligible_version(
            package,
            &metadata,
            Some(policy),
        );
    }

    fetch_un_gated_npm_latest_version(package).await
}

/// Resolve latest for the feature-disabled CLI route, where Web policy is absent.
/// Resolve latest cho route tắt feature, khi Web policy không tồn tại.
#[cfg(not(feature = "web"))]
pub(crate) async fn fetch_npm_latest_version(
    package: &str,
    _age_policy: Option<ScaffoldAgePolicy>,
) -> Result<String> {
    fetch_un_gated_npm_latest_version(package).await
}

async fn fetch_un_gated_npm_latest_version(package: &str) -> Result<String> {
    let registry_url = crate::commands::web_registry_config::web_registry_url()?;
    match fetch_npm_latest_version_from_registry(&registry_url, package).await {
        Ok(version) => Ok(version),
        Err(error) => fallback_scaffold_version(package, error, None),
    }
}

/// Keep registry failures fatal and retain the age-gated diagnostic category.
/// Giữ lỗi registry là fatal và phân loại riêng khi age gate bật.
pub(in crate::commands::core::web::implementation) fn fallback_scaffold_version(
    package: &str,
    error: anyhow::Error,
    age_policy: Option<ScaffoldAgePolicy>,
) -> Result<String> {
    if age_policy.is_some_and(|policy| policy.cutoff_hours > 0) {
        return Err(web_errors::web_scaffold_age_metadata_unavailable(
            package, &error,
        ));
    }
    Err(web_errors::web_scaffold_registry_version_unavailable(
        package, &error,
    ))
}

/// Reuse one bounded, pooled HTTP client for every CLI version lookup.
/// Dùng lại một HTTP client có pool/giới hạn cho mọi lần tra version CLI.
fn global_cli_http_client() -> &'static reqwest::Client {
    static CLIENT: std::sync::OnceLock<reqwest::Client> = std::sync::OnceLock::new();
    CLIENT.get_or_init(|| {
        reqwest::Client::builder()
            .pool_max_idle_per_host(50)
            .pool_idle_timeout(std::time::Duration::from_secs(120))
            .tcp_keepalive(std::time::Duration::from_secs(30))
            .timeout(std::time::Duration::from_secs(60))
            .user_agent(format!("MagiCore/{}", env!("CARGO_PKG_VERSION")))
            .build()
            .expect("failed to build HTTP client")
    })
}

async fn fetch_npm_latest_version_from_registry(
    registry_url: &str,
    package: &str,
) -> Result<String> {
    let selected =
        fetch_npm_selector_version_from_registry(registry_url, package, "latest").await?;
    Ok(format!("^{selected}"))
}

/// Fetch a concrete npm version for the supplied dist-tag selector.
/// Tải version npm cụ thể theo dist-tag selector.
pub(in crate::commands::core::web::implementation) async fn fetch_npm_selector_version_from_registry(
    registry_url: &str,
    package: &str,
    selector: &str,
) -> Result<String> {
    let url = format!(
        "{}/{package}/{selector}",
        registry_url.trim_end_matches('/')
    );
    let response = global_cli_http_client()
        .get(&url)
        .send()
        .await
        .map_err(|error| crate::error::network_error_fetching(package, &error))?;
    if !response.status().is_success() {
        return Err(crate::error::npm_registry_status(
            package,
            &response.status(),
        ));
    }
    let body: serde_json::Value = response
        .json()
        .await
        .map_err(|error| crate::error::bad_npm_response(package, &error))?;
    body["version"]
        .as_str()
        .map(str::to_owned)
        .ok_or_else(|| crate::error::no_version_field(package))
}

#[cfg(test)]
#[path = "test/registry_request.rs"]
mod tests;
