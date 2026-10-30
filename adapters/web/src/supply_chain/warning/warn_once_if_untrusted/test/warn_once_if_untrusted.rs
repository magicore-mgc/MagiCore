//! One-time warning state tests.
//! Test trạng thái cảnh báo một lần.

use super::super::{
    UNTRUSTED_BYPASS_WARNING, warn_each_time_if_untrusted, warn_once_if_untrusted_with,
};
use std::sync::OnceLock;

#[test]
fn warns_only_on_the_first_explicit_override() {
    let warned = OnceLock::new();

    assert!(!warn_once_if_untrusted_with(false, &warned));
    assert!(warn_once_if_untrusted_with(true, &warned));
    assert!(!warn_once_if_untrusted_with(true, &warned));
}

#[test]
fn every_bypass_use_reports_and_a_clean_run_stays_silent() {
    assert!(!warn_each_time_if_untrusted(false));
    assert!(warn_each_time_if_untrusted(true));
    assert!(warn_each_time_if_untrusted(true));
}

#[test]
fn every_use_warning_names_the_escape_hatch_and_what_is_bypassed() {
    assert!(UNTRUSTED_BYPASS_WARNING.contains("MAGICORE_ALLOW_UNTRUSTED=1"));
    assert!(UNTRUSTED_BYPASS_WARNING.contains("24h quarantine"));
    assert!(UNTRUSTED_BYPASS_WARNING.contains("BYPASSED"));
}
