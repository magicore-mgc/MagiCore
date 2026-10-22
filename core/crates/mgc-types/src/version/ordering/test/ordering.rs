//! Exercises the ordering owner at its nearest test location.
//! Kiểm tra owner ordering tại vị trí test gần nhất.

use crate::version::test_support::v;
use std::cmp::Ordering;

#[test]
fn ordering_dev_below_pre_below_final_below_post() {
    use Ordering::*;
    assert_eq!(v("1.2.3.dev1").cmp(&v("1.2.3")), Less, "dev < final");
    assert_eq!(
        v("1.2.3.dev1").cmp(&v("1.2.3-rc.1")),
        Less,
        "dev < prerelease"
    );
    assert_eq!(v("1.2.3-rc.1").cmp(&v("1.2.3")), Less, "prerelease < final");
    assert_eq!(v("1.2.3").cmp(&v("1.2.3.post1")), Less, "final < post");
    assert_eq!(
        v("1.2.3.post1").cmp(&v("1.2.3.post2")),
        Less,
        "post orders numerically"
    );
    assert_eq!(v("1.2.3.post1").cmp(&v("1.2.3.post1")), Equal);
}

#[test]
fn equal_versions_are_equal() {
    assert_eq!(v("1.0.0"), v("1.0.0"));
    assert_eq!(v("1.0.0").cmp(&v("1.0.0")), Ordering::Equal);
}

#[test]
fn stable_greater_than_prerelease() {
    let stable = v("1.0.0");
    let prerelease = v("1.0.0-next.24");
    assert!(stable > prerelease);
}

#[test]
fn numeric_prerelease_segments_sort_numerically() {
    let earlier = v("1.0.0-next.9");
    let later = v("1.0.0-next.24");
    assert!(later > earlier);
}

#[test]
fn string_prerelease_segments_sort_lexically() {
    let a = v("1.0.0-alpha");
    let b = v("1.0.0-beta");
    assert!(a < b);
}

#[test]
fn numeric_prerelease_less_than_string_prerelease() {
    // numeric fields compare less than string fields per semver spec
    let num = v("1.0.0-1");
    let string = v("1.0.0-alpha");
    assert!(num < string);
}

#[test]
fn prerelease_with_more_segments_greater() {
    let short = v("1.0.0-alpha");
    let long = v("1.0.0-alpha.1");
    assert!(short < long);
}

#[test]
fn major_version_dominates() {
    assert!(v("2.0.0") > v("1.9.9"));
}

#[test]
fn minor_version_dominates() {
    assert!(v("1.2.0") > v("1.1.9"));
}

#[test]
fn no_pre_equals_no_pre() {
    assert_eq!(v("0.0.0"), v("0.0.0"));
}

#[test]
fn both_pre_identical_are_equal() {
    assert_eq!(v("1.0.0-rc1"), v("1.0.0-rc1"));
}
