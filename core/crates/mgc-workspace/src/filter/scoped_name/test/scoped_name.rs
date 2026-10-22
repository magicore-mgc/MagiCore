//! Scoped package filter contract tests.
//! Test hợp đồng cho lọc package scoped.

#![allow(clippy::unwrap_used)]

use crate::filter_matches;
use std::path::Path;

#[test]
fn filter_scoped_prefix() {
    assert!(filter_matches("@core/*", Path::new("x"), "@core/shared"));
    assert!(filter_matches("@core/*", Path::new("x"), "@core/utils"));
    assert!(!filter_matches("@core/*", Path::new("x"), "@other/shared"));
}

#[test]
fn filter_scoped_double_star() {
    assert!(filter_matches(
        "@plugins/**",
        Path::new("x"),
        "@plugins/auth"
    ));
    assert!(filter_matches(
        "@plugins/**",
        Path::new("x"),
        "@plugins/core/db"
    ));
}
