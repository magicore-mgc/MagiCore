//! Attach RN tier ownership and derive CocoaPods registry provenance.
//! Gắn owner tier RN và tạo provenance registry CocoaPods.

use mgc_lockfile::Package;
use mgc_resolver::protocols::reactnative::RN_TIER_MARKER_PREFIX;

/// Add the tier marker once and keep the app owner-core tag.
/// Thêm marker tier đúng một lần và giữ owner-core app.
pub(in crate::native::rn_layers) fn tag_tier(mut package: Package, tier: &str) -> Package {
    package.owner_core = Some("app".to_string());
    let mut markers = package.markers.take().unwrap_or_default();
    if !markers
        .iter()
        .any(|marker| marker == &format!("{RN_TIER_MARKER_PREFIX}{tier}"))
    {
        markers.push(format!("{RN_TIER_MARKER_PREFIX}{tier}"));
    }
    package.markers = Some(markers);
    package
}

/// Derive `cocoapods://{host}` provenance from the configured CDN URL.
/// Tạo provenance `cocoapods://{host}` từ URL CDN đã cấu hình.
pub(in crate::native::rn_layers) fn cocoapods_registry_tag(cdn: &str) -> String {
    let host = cdn
        .split_once("://")
        .map(|(_, rest)| rest)
        .unwrap_or(cdn)
        .split('/')
        .next()
        .unwrap_or(cdn)
        .to_string();
    format!("cocoapods://{host}")
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
