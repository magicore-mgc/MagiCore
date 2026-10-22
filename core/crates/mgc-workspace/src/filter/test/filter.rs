//! Workspace filter coordinator contract tests.
//! Test hợp đồng cho coordinator lọc workspace.

#![allow(clippy::unwrap_used)]

use crate::filter_matches;
use std::path::Path;

#[test]
fn filter_exact_name() {
    assert!(filter_matches("lodash", Path::new("a"), "lodash"));
    assert!(!filter_matches("lodash", Path::new("a"), "other"));
}

#[test]
fn filter_empty_and_whitespace_select_all() {
    assert!(filter_matches("", Path::new("a"), "lodash"));
    assert!(filter_matches("   ", Path::new("a"), "lodash"));
}
