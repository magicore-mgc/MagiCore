//! Version/range preference rules for `mgc update` on core-web.
//! Quy tắc chọn version/range cho `mgc update` core-web.

use mgc_types::{MgResult, Version, VersionRange};

use crate::native;

/// Prefer an explicit non-prerelease `latest` dist-tag, else the maximum
/// stable version, else the raw `latest`, else the maximum of any version.
pub fn preferred_registry_version(
    metadata: &native::npm_registry::PackageMetadata,
) -> Option<String> {
    let stable_max = metadata
        .versions
        .keys()
        .filter_map(|v| Version::parse(v).ok())
        .filter(|v| v.pre.is_none())
        .max()
        .map(|v| v.to_string());

    // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
    if let Some(latest) = metadata.dist_tags.get("latest")
        && let Ok(version) = Version::parse(latest)
        && version.pre.is_none()
    {
        return Some(version.to_string());
    }

    stable_max
        .or_else(|| metadata.dist_tags.get("latest").cloned())
        .or_else(|| {
            metadata
                .versions
                .keys()
                .filter_map(|v| Version::parse(v).ok())
                .max()
                .map(|v| v.to_string())
        })
}

/// Keep the user's range style (^, ~, open, exact) while retargeting `latest`.
pub fn preferred_saved_range(current: &VersionRange, latest: &str) -> MgResult<VersionRange> {
    let raw = current.as_str();
    let next = if raw.starts_with('^') {
        format!("^{latest}")
    } else if raw.starts_with('~') {
        format!("~{latest}")
    } else if raw == "*"
        || raw.is_empty()
        || raw.starts_with(">=")
        || raw.starts_with('>')
        || raw.starts_with("<=")
    {
        format!("^{latest}")
    } else {
        latest.to_string()
    };
    VersionRange::parse(&next)
}
