//! Resolve a scaffold seed while retaining explicit compatible ranges.
//! Resolve seed scaffold và giữ nguyên range tương thích do người dùng chọn.

use super::super::ScaffoldAgePolicy;
use anyhow::Result;

/// Resolve or validate a seed version without bypassing an armed age gate.
/// Resolve hoặc kiểm tra version seed mà không bỏ qua age gate đang bật.
#[cfg(feature = "web")]
pub(crate) async fn resolve_seed_dependency_version(
    package: &str,
    preferred_range: Option<&str>,
    age_policy: Option<ScaffoldAgePolicy>,
) -> Result<String> {
    if let Some(policy) = age_policy.filter(|policy| policy.cutoff_hours > 0) {
        let metadata =
            super::super::registry_request::fetch_age_gated_npm_metadata(package).await?;
        if let Some(range) = preferred_range {
            super::super::eligible_release::validate_seed_range_age_eligible(
                package, &metadata, policy, range,
            )?;
            return Ok(range.to_string());
        }
        return super::super::eligible_release::select_latest_eligible_version(
            package,
            &metadata,
            Some(policy),
        );
    }

    if let Some(range) = preferred_range {
        return Ok(range.to_string());
    }
    super::super::registry_request::fetch_npm_latest_version(package, None).await
}

/// Keep feature-disabled builds independent of the Web adapter.
/// Giữ build tắt Web không phụ thuộc adapter Web.
#[cfg(not(feature = "web"))]
pub(crate) async fn resolve_seed_dependency_version(
    package: &str,
    preferred_range: Option<&str>,
    _age_policy: Option<ScaffoldAgePolicy>,
) -> Result<String> {
    if let Some(range) = preferred_range {
        return Ok(range.to_string());
    }
    super::super::registry_request::fetch_npm_latest_version(package, None).await
}

#[cfg(test)]
#[path = "test/seed_resolution.rs"]
mod tests;
