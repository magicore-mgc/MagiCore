//! Resolve a user-selected dist-tag without bypassing age eligibility.
//! Resolve dist-tag do người dùng chọn mà không bỏ qua age eligibility.

use super::super::{ScaffoldAgePolicy, error};
use anyhow::Result;

/// Resolve an explicitly selected npm dist-tag to its concrete package version.
/// Chuyển npm dist-tag được chọn thành version cụ thể của package.
#[cfg(feature = "web")]
pub(crate) async fn resolve_npm_dist_tag_version(
    package: &str,
    tag: &str,
    age_policy: Option<ScaffoldAgePolicy>,
) -> Result<String> {
    let age_policy = age_policy.filter(|policy| policy.cutoff_hours > 0);
    let metadata = if age_policy.is_some() {
        super::super::registry_request::fetch_age_gated_npm_metadata(package).await?
    } else {
        let registry_url = crate::commands::web_registry_config::web_registry_url()?;
        mgc_web_adapter::native::npm_registry::NpmRegistry::new(&registry_url)
            .fetch_metadata(package)
            .await
            .map_err(|error| crate::error::network_error_fetching(package, &error))?
    };
    selected_dist_tag_version(package, tag, &metadata, age_policy)
}

/// Keep the registry-based selector available in feature-disabled CLI builds.
/// Giữ selector registry khả dụng khi CLI build tắt feature Web.
#[cfg(not(feature = "web"))]
pub(crate) async fn resolve_npm_dist_tag_version(
    package: &str,
    tag: &str,
    _age_policy: Option<ScaffoldAgePolicy>,
) -> Result<String> {
    let registry_url = crate::commands::web_registry_config::web_registry_url()?;
    super::super::registry_request::fetch_npm_selector_version_from_registry(
        &registry_url,
        package,
        tag,
    )
    .await
}

/// Resolve a tag and enforce the active release-age policy.
/// Resolve tag và áp dụng release-age policy đang bật.
#[cfg(feature = "web")]
pub(in crate::commands::core::web::implementation) fn selected_dist_tag_version(
    package: &str,
    tag: &str,
    metadata: &mgc_web_adapter::native::npm_registry::PackageMetadata,
    age_policy: Option<ScaffoldAgePolicy>,
) -> Result<String> {
    let tagged_version = metadata
        .dist_tags
        .get(tag)
        .ok_or_else(|| error::web_scaffold_dist_tag_missing(package, tag))?;
    let version = mgc_types::Version::parse(tagged_version)
        .map_err(|_| error::web_scaffold_dist_tag_invalid(package, tag))?;

    if let Some(policy) = age_policy.filter(|policy| policy.cutoff_hours > 0) {
        let package_name = mgc_types::PackageName::new(package)?;
        let eligible = mgc_web_adapter::provider::eligible_versions(
            &package_name,
            metadata,
            Some(policy.into()),
        )
        .map_err(anyhow::Error::msg)?;
        if !eligible.contains(&version) {
            return Err(error::web_scaffold_dist_tag_not_eligible(
                package, tag, &version,
            ));
        }
    }

    Ok(version.to_string())
}

#[cfg(test)]
#[path = "test/dist_tag.rs"]
mod tests;
