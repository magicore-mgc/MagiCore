//! Partial package-name wildcard contract tests.
//! Test hợp đồng cho wildcard một phần của tên package.

#![allow(clippy::unwrap_used)]

use crate::filter_matches;
use std::path::Path;

#[test]
fn filter_name_wildcard_patterns() {
    assert!(filter_matches(
        "*-service",
        Path::new("any"),
        "user-service"
    ));
    assert!(filter_matches(
        "*-service",
        Path::new("any"),
        "auth-service"
    ));
    assert!(!filter_matches(
        "*-service",
        Path::new("any"),
        "user-client"
    ));
    assert!(filter_matches("mgc-*", Path::new("any"), "mgc-workspace"));
    assert!(filter_matches("*", Path::new("any"), "anything"));
    assert!(filter_matches("**", Path::new("any"), "anything"));
}
