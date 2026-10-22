//! Cross-owner Windows process scan integration tests.
//! Kiểm thử tích hợp scanner Windows qua nhiều owner.

#[test]
fn windows_metadata_retry_rewalks_new_descendants() {
    use std::collections::VecDeque;
    use std::ffi::OsString;

    let snapshot = |pid, parent_pid, creation_time, image_name: &str, command: &[&str]| {
        super::WindowsProcessSnapshot {
            pid,
            parent_pid,
            creation_time: Some(creation_time),
            image_name: image_name.to_owned(),
            command: command.iter().map(OsString::from).collect(),
        }
    };
    let snapshots = VecDeque::from([
        vec![
            snapshot(100, None, 1, "runner.exe", &["runner.exe"]),
            snapshot(101, Some(100), 2, "node.exe", &[]),
        ],
        vec![
            snapshot(100, None, 1, "runner.exe", &["runner.exe"]),
            snapshot(102, Some(101), 3, "npm.exe", &["npm.exe", "install"]),
        ],
    ]);
    let mut snapshots = snapshots;
    let mut inspections = 0;
    let mut ancestry = super::WindowsProcessAncestry::default();

    let found = super::retry_windows_process_scan(
        std::time::Instant::now(),
        None,
        || Ok(false),
        || {
            inspections += 1;
            let snapshot = snapshots.pop_front().expect("retry must inspect again");
            super::scan_windows_process_snapshot(100, &mut ancestry, &snapshot, &[], |_| Ok(None))
        },
    )
    .expect("process snapshot inspection should succeed");

    assert_eq!(inspections, 2, "the retry must refresh and rescan the tree");
    let super::WindowsProcessScan::Forbidden(found) = found else {
        panic!("npm below an exited launcher must still be rejected");
    };
    assert_eq!(found.pid, 102);
    assert_eq!(found.name, "npm");
}
