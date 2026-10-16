//! `web/versions.rs` — Web scaffold npm version resolution (P1 split
//! from web.rs, Tech Lead 2026-09-11). One responsibility: resolve the
//! current compatible version for a scaffold package from the live npm registry.
//! Explicit user selections and compatibility ranges remain supported.
//! `web/versions.rs` — Giải quyết phiên bản npm cho web scaffold (tách
//! từ web.rs theo P1). Policy tuổi phát hành được áp dụng trước; đường
//! age-gated không fallback sang phiên bản chưa xác minh.

use std::path::Path;

#[cfg(feature = "web")]
use anyhow::Context;
use anyhow::Result;
#[cfg(feature = "web")]
use mgc_types::{PackageName, Version, VersionRange};
#[cfg(feature = "web")]
use mgc_web_adapter::native::npm_registry::{NpmRegistry, PackageMetadata};
#[cfg(feature = "web")]
use mgc_web_adapter::provider::{AgePolicy, eligible_versions};

/// Web scaffold's normalized age gate, independent of optional adapter crates.
/// Age gate scaffold Web đã chuẩn hóa, không phụ thuộc adapter optional.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ScaffoldAgePolicy {
    pub cutoff_hours: u64,
    pub allow_missing_time: bool,
}

#[cfg(feature = "web")]
impl From<ScaffoldAgePolicy> for AgePolicy {
    fn from(policy: ScaffoldAgePolicy) -> Self {
        Self {
            cutoff_hours: policy.cutoff_hours,
            allow_missing_time: policy.allow_missing_time,
        }
    }
}

/// Resolve scaffold versions through the same active age policy as installation.
/// Giải quyết version scaffold theo cùng age policy đang áp dụng lúc cài đặt.
#[cfg(feature = "web")]
pub(crate) fn scaffold_age_policy(project_root: &Path) -> Result<Option<ScaffoldAgePolicy>> {
    let security = mgc_config::project::load_security_config(project_root).map_err(|error| {
        crate::error::web_scaffold_security_config_unreadable(project_root, &error)
    })?;
    let allow_missing_time = security
        .as_ref()
        .and_then(|config| config.allow_missing_time)
        .unwrap_or(false);
    if !env_truthy("MAGICORE_ALLOW_UNTRUSTED") {
        if let Some(cutoff_hours) = security
            .as_ref()
            .and_then(|config| config.min_age_for_ecosystem("web"))
        {
            return Ok(Some(ScaffoldAgePolicy {
                cutoff_hours,
                allow_missing_time,
            }));
        }
        if let Some(min_age_secs) = store_release_policy(project_root)? {
            return Ok(Some(ScaffoldAgePolicy {
                cutoff_hours: min_age_secs.div_ceil(60 * 60),
                allow_missing_time,
            }));
        }
        if env_truthy("MAGICORE_SECURITY_24H_BLOCK") || env_truthy("MGC_AUDIT_STRICT") {
            return Ok(Some(ScaffoldAgePolicy {
                cutoff_hours: mgc_config::DEFAULT_MIN_RELEASE_AGE_HOURS,
                allow_missing_time,
            }));
        }
    }
    Ok(None)
}

#[cfg(not(feature = "web"))]
pub(crate) fn scaffold_age_policy(_project_root: &Path) -> Result<Option<ScaffoldAgePolicy>> {
    Ok(None)
}

#[cfg(feature = "web")]
pub(super) fn store_release_policy(project_root: &Path) -> Result<Option<u64>> {
    let layout = mgc_store::Layout::new(mgc_web_adapter::lockfile::project_cache_dir(project_root));
    let database_path = layout.db_path();
    match std::fs::symlink_metadata(&database_path) {
        Ok(metadata) if metadata.file_type().is_file() => {}
        Ok(_) => {
            return Err(crate::error::web_scaffold_store_policy_path_invalid(
                &database_path,
            ));
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => {
            return Err(crate::error::web_scaffold_store_policy_unreadable(
                &database_path,
                &error,
            ));
        }
    }
    let database = mgc_store::Database::open(&database_path).map_err(|error| {
        crate::error::web_scaffold_store_policy_unreadable(&database_path, &error)
    })?;
    database
        .release_policy("web")
        .map_err(|error| crate::error::web_scaffold_store_policy_unreadable(&database_path, &error))
}

#[cfg(feature = "web")]
fn env_truthy(name: &str) -> bool {
    std::env::var(name)
        .ok()
        .map(|value| value.trim().to_ascii_lowercase())
        .is_some_and(|value| matches!(value.as_str(), "1" | "true" | "yes" | "on"))
}

/// Select the newest stable registry release that survives the project's age gate.
/// Chọn bản stable mới nhất còn thỏa cổng tuổi phát hành của project.
#[cfg(feature = "web")]
pub(crate) fn select_latest_eligible_version(
    package: &str,
    metadata: &PackageMetadata,
    age_policy: Option<ScaffoldAgePolicy>,
) -> Result<String> {
    let latest_tag = metadata
        .dist_tags
        .get("latest")
        .ok_or_else(|| crate::error::web_scaffold_latest_tag_missing(package))?;
    let latest = Version::parse(latest_tag)
        .with_context(|| crate::error::web_scaffold_latest_tag_invalid(package))?;
    let package_name = PackageName::new(package)?;
    let versions = eligible_versions(&package_name, metadata, age_policy.map(Into::into))
        .map_err(anyhow::Error::msg)?;
    versions
        .into_iter()
        .filter(|version| {
            version.pre.is_none()
                && version.dev.is_none()
                && version.post.is_none()
                && version <= &latest
        })
        .max()
        .map(|version| format!("^{version}"))
        .ok_or_else(|| crate::error::web_scaffold_no_eligible_release(package))
}

/// Keep a curated scaffold range only when it contains an eligible stable release.
/// Chỉ giữ range seed đã chọn khi nó còn chứa bản stable hợp lệ theo age gate.
#[cfg(feature = "web")]
pub(crate) fn validate_seed_range_age_eligible(
    package: &str,
    metadata: &PackageMetadata,
    age_policy: ScaffoldAgePolicy,
    requested_range: &str,
) -> Result<()> {
    let latest_tag = metadata
        .dist_tags
        .get("latest")
        .ok_or_else(|| crate::error::web_scaffold_latest_tag_missing(package))?;
    let latest = Version::parse(latest_tag)
        .with_context(|| crate::error::web_scaffold_latest_tag_invalid(package))?;
    let package_name = PackageName::new(package)?;
    let range = VersionRange::parse(requested_range)?;
    let versions = eligible_versions(&package_name, metadata, Some(age_policy.into()))
        .map_err(anyhow::Error::msg)?;
    if versions.into_iter().any(|version| {
        version.pre.is_none()
            && version.dev.is_none()
            && version.post.is_none()
            && version <= latest
            && range.matches(&version)
    }) {
        return Ok(());
    }
    Err(crate::error::web_scaffold_no_eligible_release_in_range(
        package,
        requested_range,
    ))
}

#[cfg(feature = "web")]
async fn fetch_age_gated_npm_metadata(package: &str) -> Result<PackageMetadata> {
    let registry_url = crate::commands::web_registry_config::web_registry_url();
    let registry = NpmRegistry::new(&registry_url);
    registry.set_age_gate_armed(true);
    registry
        .fetch_metadata(package)
        .await
        .map_err(|error| crate::error::web_scaffold_age_metadata_unavailable(package, &error))
}

/// Resolve or validate a scaffold seed version, preserving curated compatible ranges.
/// Resolve hoặc kiểm tra version seed, giữ nguyên range tương thích đã tuyển chọn.
#[cfg(feature = "web")]
pub(crate) async fn resolve_seed_dependency_version(
    package: &str,
    preferred_range: Option<&str>,
    age_policy: Option<ScaffoldAgePolicy>,
) -> Result<String> {
    if let Some(policy) = age_policy.filter(|policy| policy.cutoff_hours > 0) {
        let metadata = fetch_age_gated_npm_metadata(package).await?;
        if let Some(range) = preferred_range {
            validate_seed_range_age_eligible(package, &metadata, policy, range)?;
            return Ok(range.to_string());
        }
        return select_latest_eligible_version(package, &metadata, Some(policy));
    }

    if let Some(range) = preferred_range {
        return Ok(range.to_string());
    }
    fetch_npm_latest_version(package, None).await
}

/// Resolve an explicitly selected npm dist-tag to its concrete package version.
/// Chuyển dist-tag do người dùng chọn thành version cụ thể của package.
#[cfg(feature = "web")]
pub(crate) async fn resolve_npm_dist_tag_version(
    package: &str,
    tag: &str,
    age_policy: Option<ScaffoldAgePolicy>,
) -> Result<String> {
    let age_policy = age_policy.filter(|policy| policy.cutoff_hours > 0);
    let metadata = if age_policy.is_some() {
        fetch_age_gated_npm_metadata(package).await?
    } else {
        let registry_url = crate::commands::web_registry_config::web_registry_url();
        NpmRegistry::new(&registry_url)
            .fetch_metadata(package)
            .await
            .map_err(|error| crate::error::network_error_fetching(package, &error))?
    };
    selected_dist_tag_version(package, tag, &metadata, age_policy)
}

#[cfg(feature = "web")]
pub(super) fn selected_dist_tag_version(
    package: &str,
    tag: &str,
    metadata: &PackageMetadata,
    age_policy: Option<ScaffoldAgePolicy>,
) -> Result<String> {
    let tagged_version = metadata
        .dist_tags
        .get(tag)
        .ok_or_else(|| crate::error::web_scaffold_dist_tag_missing(package, tag))?;
    let version = Version::parse(tagged_version)
        .map_err(|_| crate::error::web_scaffold_dist_tag_invalid(package, tag))?;

    if let Some(policy) = age_policy.filter(|policy| policy.cutoff_hours > 0) {
        let package_name = PackageName::new(package)?;
        let eligible = eligible_versions(&package_name, metadata, Some(policy.into()))
            .map_err(anyhow::Error::msg)?;
        if !eligible.contains(&version) {
            return Err(crate::error::web_scaffold_dist_tag_not_eligible(
                package, tag, &version,
            ));
        }
    }

    Ok(version.to_string())
}

#[cfg(not(feature = "web"))]
pub(crate) async fn resolve_npm_dist_tag_version(
    package: &str,
    tag: &str,
    _age_policy: Option<ScaffoldAgePolicy>,
) -> Result<String> {
    let registry_url = crate::commands::web_registry_config::web_registry_url();
    fetch_npm_selector_version_from_registry(&registry_url, package, tag).await
}

#[cfg(not(feature = "web"))]
pub(crate) async fn resolve_seed_dependency_version(
    package: &str,
    preferred_range: Option<&str>,
    _age_policy: Option<ScaffoldAgePolicy>,
) -> Result<String> {
    if let Some(range) = preferred_range {
        return Ok(range.to_string());
    }
    fetch_npm_latest_version(package, None).await
}

#[cfg(feature = "web")]
pub(crate) async fn fetch_npm_latest_version(
    package: &str,
    age_policy: Option<ScaffoldAgePolicy>,
) -> Result<String> {
    if let Some(policy) = age_policy.filter(|policy| policy.cutoff_hours > 0) {
        let metadata = fetch_age_gated_npm_metadata(package).await?;
        return select_latest_eligible_version(package, &metadata, Some(policy));
    }

    fetch_un_gated_npm_latest_version(package).await
}

#[cfg(not(feature = "web"))]
pub(crate) async fn fetch_npm_latest_version(
    package: &str,
    _age_policy: Option<ScaffoldAgePolicy>,
) -> Result<String> {
    fetch_un_gated_npm_latest_version(package).await
}

async fn fetch_un_gated_npm_latest_version(package: &str) -> Result<String> {
    let registry_url = crate::commands::web_registry_config::web_registry_url();
    match fetch_npm_latest_version_from_registry(&registry_url, package).await {
        Ok(version) => Ok(version),
        Err(error) => fallback_scaffold_version(package, error, None),
    }
}

pub(super) fn fallback_scaffold_version(
    package: &str,
    error: anyhow::Error,
    age_policy: Option<ScaffoldAgePolicy>,
) -> Result<String> {
    if age_policy.is_some_and(|policy| policy.cutoff_hours > 0) {
        return Err(crate::error::web_scaffold_age_metadata_unavailable(
            package, &error,
        ));
    }
    Err(crate::error::web_scaffold_registry_version_unavailable(
        package, &error,
    ))
}

/// Shared HTTP client (pooled — one client for every version lookup).
/// HTTP client dùng chung (pool — một client cho mọi lần tra version).
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

async fn fetch_npm_selector_version_from_registry(
    registry_url: &str,
    package: &str,
    selector: &str,
) -> Result<String> {
    let url = format!(
        "{}/{package}/{selector}",
        registry_url.trim_end_matches('/')
    );
    let resp = global_cli_http_client()
        .get(&url)
        .send()
        .await
        .map_err(|e| crate::error::network_error_fetching(package, &e))?;
    if !resp.status().is_success() {
        return Err(crate::error::npm_registry_status(package, &resp.status()));
    }
    let body: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| crate::error::bad_npm_response(package, &e))?;
    body["version"]
        .as_str()
        .map(str::to_owned)
        .ok_or_else(|| crate::error::no_version_field(package))
}
