//! Filter registry versions to stable, latest-compatible, age-eligible releases.
//! Lọc version registry thành release stable, không mới hơn latest và đạt age gate.

use super::super::{ScaffoldAgePolicy, error};
use anyhow::{Context, Result};
use mgc_types::{PackageName, Version, VersionRange};
use mgc_web_adapter::native::npm_registry::PackageMetadata;
use mgc_web_adapter::provider::eligible_versions;

/// Select the newest stable release that survives the project's age gate.
/// Chọn stable release mới nhất còn thỏa cổng tuổi của project.
pub(crate) fn select_latest_eligible_version(
    package: &str,
    metadata: &PackageMetadata,
    age_policy: Option<ScaffoldAgePolicy>,
) -> Result<String> {
    let latest_tag = metadata
        .dist_tags
        .get("latest")
        .ok_or_else(|| error::web_scaffold_latest_tag_missing(package))?;
    let latest = Version::parse(latest_tag)
        .with_context(|| error::web_scaffold_latest_tag_invalid(package))?;
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
        .ok_or_else(|| error::web_scaffold_no_eligible_release(package))
}

/// Keep a curated scaffold range only when it contains an eligible stable release.
/// Chỉ giữ range scaffold đã tuyển khi nó còn chứa stable release hợp lệ.
pub(crate) fn validate_seed_range_age_eligible(
    package: &str,
    metadata: &PackageMetadata,
    age_policy: ScaffoldAgePolicy,
    requested_range: &str,
) -> Result<()> {
    let latest_tag = metadata
        .dist_tags
        .get("latest")
        .ok_or_else(|| error::web_scaffold_latest_tag_missing(package))?;
    let latest = Version::parse(latest_tag)
        .with_context(|| error::web_scaffold_latest_tag_invalid(package))?;
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
    Err(error::web_scaffold_no_eligible_release_in_range(
        package,
        requested_range,
    ))
}

#[cfg(test)]
#[path = "test/eligible_release.rs"]
mod tests;
