//! One-time behavior of the missing-publish-time warning.
//! Hành vi một lần của cảnh báo timestamp phát hành thiếu.

use super::super::warn_once_if_missing_publish_time_with;
use std::sync::OnceLock;

#[test]
fn warns_only_on_first_explicit_missing_time_acceptance() {
    let warned = OnceLock::new();

    assert!(warn_once_if_missing_publish_time_with(&warned));
    assert!(!warn_once_if_missing_publish_time_with(&warned));
}
