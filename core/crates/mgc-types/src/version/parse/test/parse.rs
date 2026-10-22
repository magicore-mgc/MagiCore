//! Exercises the parse owner at its nearest test location.
//! Kiểm tra owner parse tại vị trí test gần nhất.

use crate::version::Version;
use crate::version::test_support::v;

#[test]
fn parse_v_prefix_stripped() {
    assert_eq!(v("v1.2.3"), v("1.2.3"));
}

#[test]
fn parse_missing_minor_defaults_zero() {
    let v = v("1");
    assert_eq!(v.major, 1);
    assert_eq!(v.minor, 0);
    assert_eq!(v.patch, 0);
    assert!(v.pre.is_none());
}

#[test]
fn parse_missing_patch_defaults_zero() {
    let v = v("1.2");
    assert_eq!(v.major, 1);
    assert_eq!(v.minor, 2);
    assert_eq!(v.patch, 0);
}

#[test]
fn parse_with_prerelease() {
    let v = v("1.2.3-rc.1");
    assert_eq!(v.pre.as_deref(), Some("rc.1"));
}

#[test]
fn malformed_build_metadata_is_rejected() {
    assert!(Version::parse("1.0.0+").is_err());
    assert!(Version::parse("1.0.0+bad..metadata").is_err());
    assert!(Version::parse("1.0.0+bad metadata").is_err());
}

#[test]
fn parse_empty_returns_err() {
    assert!(Version::parse("").is_err());
}

#[test]
fn parse_invalid_chars_returns_err() {
    assert!(Version::parse("1.x").is_err());
    assert!(Version::parse("a.b.c").is_err());
}

#[test]
fn parse_whitespace_trimmed() {
    assert_eq!(v("  1.2.3  "), v("1.2.3"));
}

#[test]
fn parse_v_only_returns_err() {
    assert!(Version::parse("v").is_err());
}

#[test]
fn parse_post_release_keeps_post_number() {
    let v = v("1.2.3.post1");
    assert_eq!((v.major, v.minor, v.patch), (1, 2, 3));
    assert_eq!(v.post, Some(1));
    assert_eq!(v.dev, None);
    assert_eq!(v.to_string(), "1.2.3.post1");
}

#[test]
fn parse_dev_release_keeps_dev_number() {
    let v = v("2.0.dev0");
    assert_eq!(v.dev, Some(0));
    assert_eq!(v.post, None);
    assert_eq!(v.to_string(), "2.0.0.dev0");
}

#[test]
fn parse_bare_post_defaults_to_zero() {
    assert_eq!(v("1.2.3.post").post, Some(0));
}

#[test]
fn parse_extra_segments_rejects_loudly() {
    // Previously "1.2.3.post1" silently became "1.2.3" — truncation,
    // not leniency. Unknown shapes must fail closed.
    assert!(Version::parse("1.2.3.4").is_err());
    assert!(Version::parse("1.2.3.foo").is_err());
    assert!(Version::parse("1.2.3.postx").is_err());
    assert!(Version::parse("1.2.3.post1.2").is_err());
}
