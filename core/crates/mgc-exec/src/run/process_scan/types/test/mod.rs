//! Shared snapshot type tests.
//! Test type snapshot dùng chung.

use super::*;

#[test]
fn snapshot_retains_parent_group_start_identity_and_zombie_state() {
    let snapshot = ProcessScanSnapshot {
        parent_pid: Some(10),
        process_group: Some(12),
        #[cfg(not(target_os = "linux"))]
        start_time_seconds: 100,
        #[cfg(not(target_os = "linux"))]
        start_time_subseconds: 5,
        #[cfg(target_os = "linux")]
        process_start_ticks: 105,
        is_zombie: true,
    };
    let copied = snapshot;
    assert_eq!(copied.parent_pid, Some(10));
    assert_eq!(copied.process_group, Some(12));
    assert!(copied.is_zombie);
    #[cfg(target_os = "linux")]
    assert_eq!(copied.process_start_ticks, 105);
    #[cfg(not(target_os = "linux"))]
    assert_eq!(
        (copied.start_time_seconds, copied.start_time_subseconds),
        (100, 5)
    );
}
