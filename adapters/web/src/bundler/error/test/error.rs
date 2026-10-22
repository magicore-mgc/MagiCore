//! Tests for stable bundler error display.
//! Test display lỗi bundler ổn định.

use super::super::{esbuild_failed, link_node_modules_failed};
use std::path::Path;

#[test]
fn esbuild_failure_keeps_the_existing_prefix_and_joiner() {
    let error = esbuild_failed(&["first".to_string(), "second".to_string()]);

    assert_eq!(error.to_string(), "esbuild failed: first\nsecond");
}

#[test]
fn node_modules_link_failure_keeps_the_existing_path_message() {
    let error = link_node_modules_failed(
        Path::new("source"),
        Path::new("target"),
        &std::io::Error::other("denied"),
    );

    assert_eq!(
        error.to_string(),
        "failed to link node_modules for build from source to target: denied"
    );
}
