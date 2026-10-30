//! Linux process snapshot tests — PID identity, zombie state, and disappearing proc entries.
//! (Kiểm tra parser cùng ranh giới bỏ qua race process đã thoát.)

use super::*;

#[cfg(target_os = "linux")]
#[test]
fn linux_process_stat_keeps_exact_identity_when_pid_is_reused_within_one_second() {
    let stat = |ticks| {
        format!(
            "42 (go (worker) with spaces) S 1 7 7 0 -1 4194304 0 0 0 0 0 0 0 0 20 0 1 0 {ticks}"
        )
    };
    let original = super::parse_linux_process_stat(42, &stat(12345))
        .expect("parse Linux proc stat with parenthesized comm");
    let reused = super::parse_linux_process_stat(42, &stat(12346))
        .expect("parse Linux proc stat after PID reuse");
    assert_eq!(original.parent_pid, Some(1));
    assert_eq!(original.process_group, Some(7));
    assert_eq!(original.process_start_ticks, 12345);
    assert!(!original.is_zombie);
    // The identity helper lives under the monitor owner, one branch over.
    // (Helper identity nằm dưới owner monitor, ở nhánh khác.)
    assert!(!super::super::monitor::process_identity_is_live(
        original, reused
    ));

    let zombie = super::parse_linux_process_stat(
        42,
        "42 (go worker) Z 1 7 7 0 -1 4194304 0 0 0 0 0 0 0 0 20 0 1 0 12347",
    )
    .expect("parse Linux zombie process state");
    assert!(zombie.is_zombie);
    assert!(super::parse_linux_process_stat(43, &stat(12345)).is_none());
}

#[cfg(target_os = "linux")]
#[test]
fn linux_process_scan_skips_only_an_entry_with_missing_pid_directory() {
    let root = tempfile::tempdir().expect("create proc-entry fixture");
    let process_dir = root.path().join("4242");
    std::fs::create_dir(&process_dir).expect("create simulated proc entry");

    assert!(
        !super::linux_proc_process_dir_is_gone(&process_dir)
            .expect("inspect still-present proc entry")
    );
    assert!(super::parse_linux_process_stat_entry(4242, "malformed stat", &process_dir).is_err());

    std::fs::remove_dir(&process_dir).expect("remove simulated exited process");
    assert!(
        super::linux_proc_process_dir_is_gone(&process_dir)
            .expect("recognize process disappearance")
    );
    assert!(
        super::parse_linux_process_stat_entry(4242, "malformed stat", &process_dir)
            .expect("skip a process that exited during stat parsing")
            .is_none()
    );
}
