#![cfg(test)]
#![allow(clippy::unwrap_used)]

//! Verify idempotent tier markers and normalized CDN host provenance.
//! Kiểm tra marker tier idempotent và provenance host CDN.

use crate::native::rn_layers::lock_markers::lock_markers::{cocoapods_registry_tag, tag_tier};
use mgc_lockfile::Package;
use mgc_resolver::protocols::reactnative::RN_TIER_MARKER_PREFIX;

#[test]
fn tag_tier_adds_marker_only_once() {
    let package = tag_tier(Package::default(), "android");
    let package = tag_tier(package, "android");
    assert_eq!(package.owner_core.as_deref(), Some("app"));
    assert_eq!(
        package.markers.unwrap(),
        vec![format!("{RN_TIER_MARKER_PREFIX}android")]
    );
}

#[test]
fn cocoapods_provenance_keeps_cdn_host_only() {
    assert_eq!(
        cocoapods_registry_tag("https://cdn.cocoapods.org/path"),
        "cocoapods://cdn.cocoapods.org"
    );
}
