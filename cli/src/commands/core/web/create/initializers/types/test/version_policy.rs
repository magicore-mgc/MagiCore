//! Version policy selector tests.
//! Test selector của chính sách phiên bản.

use super::super::VersionPolicy;

#[test]
fn latest_requests_the_dist_tag() {
    assert_eq!(VersionPolicy::Latest.selector(), "latest");
}

#[test]
fn pinned_requests_the_exact_version() {
    assert_eq!(VersionPolicy::Pinned("15.0.0").selector(), "15.0.0");
}

#[test]
fn compatible_requests_the_declared_range() {
    assert_eq!(VersionPolicy::Compatible("^15.0.0").selector(), "^15.0.0");
}
