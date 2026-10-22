//! Select configured policy channels and preserve URL-first fallback.
//! Chọn kênh policy đã cấu hình và giữ fallback URL trước.

use std::path::Path;

/// Environment variable selecting URL distribution for manifests.
/// Env chọn phân phối manifest qua URL.
pub const POLICY_MANIFEST_URL_ENV: &str = "MGC_POLICY_MANIFEST_URL";

/// Environment variable selecting OCI-registry distribution.
/// Env chọn phân phối qua OCI registry.
pub const POLICY_MANIFEST_OCI_ENV: &str = "MGC_POLICY_MANIFEST_OCI";

/// Optional bearer token for a private OCI registry.
/// Token Bearer cho OCI registry private.
pub const POLICY_OCI_TOKEN_ENV: &str = "MGC_POLICY_OCI_TOKEN";

/// Media type of the OCI config blob carrying the policy document.
/// Media type của config blob OCI chứa tài liệu policy.
pub const POLICY_OCI_CONFIG_MEDIA_TYPE: &str = "application/vnd.magicore.policy.exceptions.v2+json";

#[derive(Debug, PartialEq, Eq)]
enum ManifestDistributionChannel {
    Url(String),
    Oci(String),
}

fn configured_channels(
    url: Option<String>,
    oci: Option<String>,
) -> Vec<ManifestDistributionChannel> {
    let mut channels = Vec::with_capacity(2);
    if let Some(url) = url {
        channels.push(ManifestDistributionChannel::Url(url));
    }
    if let Some(oci) = oci {
        channels.push(ManifestDistributionChannel::Oci(oci));
    }
    channels
}

/// URL distribution refresh when configured (Spec rev4 §16.7 — decided):
/// fetch → verify → atomically refresh the data-dir cache, then file
/// loading sees it. Failures warn and fall back to the file cache.
/// (Refresh URL đã cấu hình: tải → xác minh → thay cache data-dir; lỗi thì
/// cảnh báo và dùng file cache.)
pub async fn refresh_url_manifest_if_configured(_project_root: &Path) {
    let url = std::env::var_os(POLICY_MANIFEST_URL_ENV)
        .filter(|value| !value.is_empty())
        .map(|value| value.to_string_lossy().into_owned());
    let oci = std::env::var_os(POLICY_MANIFEST_OCI_ENV)
        .filter(|value| !value.is_empty())
        .map(|value| value.to_string_lossy().into_owned());
    let channels = configured_channels(url, oci);
    if channels.is_empty() {
        return;
    }
    let has_url = channels
        .iter()
        .any(|channel| matches!(channel, ManifestDistributionChannel::Url(_)));
    let has_oci = channels
        .iter()
        .any(|channel| matches!(channel, ManifestDistributionChannel::Oci(_)));
    let trusted = super::super::manifest_load::trusted_policy_key_ids();
    if trusted.is_empty() {
        mgc_ui::warning(
            "policy manifest distribution is configured but MGC_POLICY_TRUST_KEYS is empty; skipping refresh",
        );
        return;
    }
    if has_url && has_oci {
        mgc_ui::warning(
            "both policy manifest distribution variables are set; trying HTTPS URL first and OCI only if it fails",
        );
    }
    let cache =
        mgc_platform::paths::mgc_data_dir().join(mgc_exec::policy_manifest::cache_rel_path());
    for channel in channels {
        match channel {
            ManifestDistributionChannel::Url(url) => {
                match super::super::manifest_url::refresh_manifest_from_url(&url, &trusted, &cache)
                    .await
                {
                    Ok(_) => return,
                    Err(error) => mgc_ui::warning(&format!(
                        "policy manifest URL refresh failed ({error:#}); continuing with the file cache"
                    )),
                }
            }
            ManifestDistributionChannel::Oci(reference) => {
                if let Err(error) = super::super::manifest_oci::refresh_manifest_from_oci(
                    &reference, &trusted, &cache,
                )
                .await
                {
                    mgc_ui::warning(&format!(
                        "policy OCI refresh failed ({error:#}); continuing with the file cache"
                    ));
                }
            }
        }
    }
}

#[cfg(test)]
#[path = "test/manifest_distribution.rs"]
mod tests;
