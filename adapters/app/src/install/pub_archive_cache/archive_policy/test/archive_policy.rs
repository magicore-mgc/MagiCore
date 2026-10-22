//! Archive policy boundary tests.
//! Test biên cho chính sách archive.
#![allow(clippy::unwrap_used)]

use super::super::super::archive_policy::{ARCHIVE_MAX_BYTES_ENV, DEFAULT_ARCHIVE_MAX_BYTES};
use super::super::parse_archive_limit;

#[test]
fn default_archive_limit_is_nonzero() {
    const { assert!(DEFAULT_ARCHIVE_MAX_BYTES > 0) };
    assert_eq!(ARCHIVE_MAX_BYTES_ENV, "MGC_PUB_ARCHIVE_MAX_BYTES");
}

#[test]
fn configured_archive_limit_requires_a_positive_integer() {
    assert_eq!(parse_archive_limit("4096").unwrap(), 4096);
    assert!(parse_archive_limit("0").is_err());
    assert!(parse_archive_limit("-1").is_err());
    assert!(parse_archive_limit("invalid").is_err());
}
