//! Tests age-gated latest-version selection without registry access.
//! Kiểm tra chọn latest theo age gate mà không truy cập registry.

#![allow(clippy::unwrap_used)]

use super::super::select_latest_version;
use mgc_types::Version;

fn version(value: &str) -> Version {
    Version::parse(value).unwrap()
}

#[test]
fn eligible_latest_tag_skips_fallback_lookup() {
    let mut fallback_called = false;
    let latest = select_latest_version(Some("2.0.0"), true, || {
        fallback_called = true;
        Some(vec![version("3.0.0")])
    });
    assert_eq!(latest.as_deref(), Some("2.0.0"));
    assert!(!fallback_called);
}

#[test]
fn rejected_latest_tag_uses_the_maximum_eligible_version() {
    let latest = select_latest_version(Some("2.0.0"), false, || {
        Some(vec![version("1.8.0"), version("1.9.0")])
    });
    assert_eq!(latest.as_deref(), Some("1.9.0"));
}

#[test]
fn absent_latest_tag_uses_eligible_versions() {
    let latest = select_latest_version(None, false, || Some(vec![version("1.4.0")]));
    assert_eq!(latest.as_deref(), Some("1.4.0"));
}

#[test]
fn failed_eligibility_lookup_returns_no_latest_candidate() {
    let latest = select_latest_version(Some("2.0.0"), false, || None);
    assert_eq!(latest, None);
}
