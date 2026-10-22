//! Checks public version-range behavior at its owner.
//! Kiểm tra hành vi version range công khai tại owner gần nhất.

use super::super::VersionRange;
use crate::version::Version;

#[test]
fn matches_compound_and_union_ranges() {
    let range = VersionRange::parse(">=1.2.0 <2.0.0 || 3.0.0").unwrap();
    assert!(range.matches(&Version::parse("1.8.0").unwrap()));
    assert!(range.matches(&Version::parse("3.0.0").unwrap()));
    assert!(!range.matches(&Version::parse("2.4.0").unwrap()));
}

#[test]
fn matches_inclusive_hyphen_range_endpoints() {
    let range = VersionRange::parse("1.2.0 - 1.4.0").unwrap();
    assert!(range.matches(&Version::parse("1.2.0").unwrap()));
    assert!(range.matches(&Version::parse("1.4.0").unwrap()));
}
