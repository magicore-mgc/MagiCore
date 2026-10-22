//! Fetch and verify policy manifests from the configured URL channel.
//! Tải và xác minh manifest policy từ kênh URL.

use std::path::Path;

use super::super::manifest_distribution::error::{
    PolicyManifestDistributionError as DistributionError, PolicyManifestDistributionResult,
};

/// Explicit manifest distribution over HTTPS (Spec rev4 §16.7 — decided):
/// fetch → verify → atomically replace the data-dir cache. Plain HTTP is
/// refused except loopback; bodies over 1 MiB are refused before caching.
/// (Phân phối manifest tường minh qua HTTPS (Spec rev4 §16.7 — đã chốt):
/// tải → xác minh → thay atomically cache data-dir. HTTP thường bị từ chối
/// trừ loopback; body quá 1 MiB bị từ chối trước khi cache.)
async fn refresh_manifest_from_url_inner(
    url: &str,
    trusted_hex_ids: &[String],
    cache_path: &Path,
) -> PolicyManifestDistributionResult<mgc_exec::policy_manifest::PolicyManifest> {
    let parsed = url
        .parse::<url::Url>()
        .map_err(|source| DistributionError::InvalidUrl {
            detail: source.to_string(),
            source,
        })?;
    let is_https = parsed.scheme() == "https";
    let is_loopback = matches!(
        parsed.host_str(),
        Some("localhost") | Some("127.0.0.1") | Some("::1")
    );
    if !is_https && !is_loopback {
        return Err(DistributionError::InsecureUrl);
    }
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()?;
    let bytes = client.get(url).send().await?.bytes().await?;
    Ok(super::super::manifest_cache::verify_and_cache(
        &bytes,
        trusted_hex_ids,
        cache_path,
    )?)
}

/// Refresh a signed manifest from HTTPS or loopback HTTP.
/// Refresh manifest có ký từ HTTPS hoặc HTTP loopback.
pub async fn refresh_manifest_from_url(
    url: &str,
    trusted_hex_ids: &[String],
    cache_path: &Path,
) -> anyhow::Result<mgc_exec::policy_manifest::PolicyManifest> {
    refresh_manifest_from_url_inner(url, trusted_hex_ids, cache_path)
        .await
        .map_err(anyhow::Error::new)
}

#[cfg(test)]
#[path = "test/url.rs"]
mod tests;
