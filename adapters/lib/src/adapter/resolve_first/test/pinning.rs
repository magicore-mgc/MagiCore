//! Preserve ecosystem-specific and explicit dependency range formatting.
//! Giữ định dạng range riêng theo ecosystem và range tường minh.

use super::super::pinning::pin_requested_range;
use crate::adapter::types::ResolveFirst;
use mgc_types::{Version, VersionRange};

#[test]
fn unpinned_python_dependency_uses_exact_pep_508_pin() {
    let resolved = Version::parse("1.2.3").unwrap();
    let actual =
        pin_requested_range(&ResolveFirst::Python, VersionRange::star(), &resolved).unwrap();

    assert_eq!(actual, VersionRange::parse("==1.2.3").unwrap());
}

#[test]
fn unpinned_native_non_python_lanes_use_the_resolved_bare_version() {
    let resolved = Version::parse("1.2.3").unwrap();
    for lane in [
        ResolveFirst::Rust,
        ResolveFirst::Go,
        ResolveFirst::DotNet,
        ResolveFirst::JavaPom,
    ] {
        let actual = pin_requested_range(&lane, VersionRange::star(), &resolved).unwrap();
        assert_eq!(actual, VersionRange::parse("1.2.3").unwrap());
    }
}

#[test]
fn explicit_requested_range_is_preserved() {
    let resolved = Version::parse("1.2.3").unwrap();
    let requested = VersionRange::parse(">=1.0, <2.0").unwrap();

    let actual = pin_requested_range(&ResolveFirst::Python, requested.clone(), &resolved).unwrap();

    assert_eq!(actual, requested);
}
