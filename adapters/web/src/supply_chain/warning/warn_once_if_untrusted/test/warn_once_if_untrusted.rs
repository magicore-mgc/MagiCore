//! One-time warning state tests.
//! Test trạng thái cảnh báo một lần.

use super::super::warn_once_if_untrusted_with;
use std::sync::OnceLock;

#[test]
fn warns_only_on_the_first_explicit_override() {
    let warned = OnceLock::new();

    assert!(!warn_once_if_untrusted_with(false, &warned));
    assert!(warn_once_if_untrusted_with(true, &warned));
    assert!(!warn_once_if_untrusted_with(true, &warned));
}
