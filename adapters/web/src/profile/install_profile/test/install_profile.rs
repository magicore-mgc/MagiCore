//! Tests for install timing-profile enablement and marks.
//! Kiểm tra bật/tắt và dấu thời gian của install profile.

use super::InstallProfile;
use std::time::Instant;

#[test]
fn disabled_profile_ignores_timing_marks() {
    let mut profile = InstallProfile::default();
    profile.mark("resolve", Instant::now());
    profile.mark_step("fetch", Instant::now());
    assert!(profile.marks.is_empty());
}

#[test]
fn enabled_profile_keeps_mark_order_and_labels() {
    let mut profile = InstallProfile {
        enabled: true,
        ..Default::default()
    };
    profile.mark("resolve", Instant::now());
    profile.mark_step("fetch", Instant::now());
    let labels: Vec<_> = profile.marks.iter().map(|(label, _)| *label).collect();
    assert_eq!(labels.len(), 2);
    assert_eq!(labels[0], "resolve");
    assert_eq!(labels[1], "fetch");
}
