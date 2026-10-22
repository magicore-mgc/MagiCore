//! Relative path-glob contract tests.
//! Test hợp đồng cho glob đường dẫn tương đối.

#![allow(clippy::unwrap_used)]

use crate::filter_matches;
use std::path::Path;

#[test]
fn filter_relative_path() {
    assert!(filter_matches(
        "./apps/*",
        Path::new("apps/web-app"),
        "@core/web-app"
    ));
    assert!(!filter_matches(
        "./apps/*",
        Path::new("packages/shared"),
        "@core/shared"
    ));
    assert!(filter_matches(
        "apps/*",
        Path::new("apps/web-app"),
        "@core/web-app"
    ));
}

#[test]
fn filter_double_star() {
    assert!(filter_matches(
        "./packages/**",
        Path::new("packages/core/shared"),
        "x"
    ));
    assert!(filter_matches(
        "./packages/**",
        Path::new("packages/shared"),
        "x"
    ));
    assert!(!filter_matches("./packages/**", Path::new("apps/app"), "x"));
}
