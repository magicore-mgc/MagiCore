// macOS native snapshot test — verify current process identity is observable.
// (Kiểm tra API hệ điều hành nhận diện identity của process hiện tại.)

use super::*;

#[test]
fn current_process_snapshot_has_live_identity() {
    let pid = std::process::id();
    let snapshot =
        macos_process_snapshot(pid).expect("macOS should expose current process identity");

    assert!(snapshot.parent_pid.is_some());
    assert!(snapshot.process_group.is_some());
    assert!(!snapshot.is_zombie);
}
