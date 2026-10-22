//! Select an explicit or latest release version.
//! Chọn phiên bản phát hành được yêu cầu hoặc tag mới nhất.

use crate::commands::self_update::error::SelfUpdateError;
use anyhow::Result;

/// GitHub repo hosting the release artifacts.
pub(in crate::commands) const RELEASE_REPO: &str = "mingd-153/MagiCore";

/// Resolve the target version: explicit `--version` (with or without a
/// leading `v`), or `latest` via the releases/latest redirect (no JSON
/// parser needed — the effective URL ends in `/tag/<tag>`).
/// (Resolve version: tường minh hoặc `latest` qua redirect.)
pub async fn resolve_version(version: Option<&str>) -> Result<String> {
    match version {
        // Explicit "latest" keyword resolves like the default.
        // (Từ khóa "latest" tường minh resolve như mặc định.)
        Some("latest") | None => resolve_latest_tag().await,
        Some(v) => {
            let number = v.strip_prefix('v').unwrap_or(v);
            if !is_valid_version(number) {
                return Err(SelfUpdateError::invalid_version(v));
            }
            Ok(number.to_string())
        }
    }
}

/// Anchored semver check shared by explicit versions (same rule as the
/// install scripts — unanchored patterns accept `1.2.3evil`).
/// (Kiểm tra semver neo 2 đầu.)
pub(in crate::commands) fn is_valid_version(v: &str) -> bool {
    let mut parts = v.splitn(2, '-');
    let core = parts.next().unwrap_or("");
    let nums: Vec<&str> = core.split('.').collect();
    if nums.len() != 3
        || nums
            .iter()
            .any(|n| n.is_empty() || !n.chars().all(|c| c.is_ascii_digit()))
    {
        return false;
    }
    match parts.next() {
        None => true,
        Some(pre) => {
            !pre.is_empty()
                && pre
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-')
        }
    }
}

/// Follow `/releases/latest` to the newest published tag using the
/// shared HTTP client (timeouts + retries, never raw reqwest).
/// (Theo redirect latest bằng HTTP client chung.)
pub(super) async fn resolve_latest_tag() -> Result<String> {
    let client = mgc_http::HttpClient::default();
    let url = format!("https://github.com/{RELEASE_REPO}/releases/latest");
    let response = client
        .get(&url)
        .await
        .map_err(|e| anyhow::anyhow!("resolve latest release at {url} failed: {e}"))?;
    let effective = response.url().to_string();
    match effective.rsplit("/tag/").next() {
        Some(tag) if tag != effective && !tag.is_empty() && tag.contains('.') => {
            Ok(tag.trim_start_matches('v').to_string())
        }
        _ => anyhow::bail!(
            "could not resolve the latest release (no published releases at {url}) — re-run with an explicit --version"
        ),
    }
}
