//! Tests for resolve timing-profile enablement and marks.
//! Kiểm tra bật/tắt và dấu thời gian của resolve profile.

use super::ResolveProfile;
use std::time::Instant;

#[test]
fn disabled_profile_ignores_timing_marks() {
    let mut profile = ResolveProfile::default();
    profile.mark("lockfile", Instant::now());
    assert!(profile.marks.is_empty());
}

#[test]
fn enabled_profile_keeps_mark_order_and_labels() {
    let mut profile = ResolveProfile {
        enabled: true,
        ..Default::default()
    };
    profile.mark("lockfile", Instant::now());
    profile.mark("registry", Instant::now());
    let labels: Vec<_> = profile.marks.iter().map(|(label, _)| *label).collect();
    assert_eq!(labels.len(), 2);
    assert_eq!(labels[0], "lockfile");
    assert_eq!(labels[1], "registry");
}
