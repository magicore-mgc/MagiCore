//! Tests for materialization counters and disabled behavior.
//! Kiểm tra bộ đếm materialization và trạng thái bị tắt.

use super::MaterializationProfile;
use std::sync::atomic::Ordering;

#[test]
fn enabled_profile_counts_each_materialization_kind() {
    let profile = MaterializationProfile {
        enabled: true,
        ..Default::default()
    };
    profile.record_package_linked();
    profile.record_directory();
    profile.record_file();
    profile.record_hardlink();
    profile.record_copy();
    profile.record_reflink();
    profile.record_symlink();

    assert_eq!(profile.packages_linked.load(Ordering::Relaxed), 1);
    assert_eq!(profile.directories_seen.load(Ordering::Relaxed), 1);
    assert_eq!(profile.files_seen.load(Ordering::Relaxed), 1);
    assert_eq!(profile.hardlinks.load(Ordering::Relaxed), 1);
    assert_eq!(profile.copies.load(Ordering::Relaxed), 1);
    assert_eq!(profile.reflinks.load(Ordering::Relaxed), 1);
    assert_eq!(profile.symlinks.load(Ordering::Relaxed), 1);
}

#[test]
fn disabled_profile_does_not_increment_counters() {
    let profile = MaterializationProfile::default();
    profile.record_file();
    assert_eq!(profile.files_seen.load(Ordering::Relaxed), 0);
}
