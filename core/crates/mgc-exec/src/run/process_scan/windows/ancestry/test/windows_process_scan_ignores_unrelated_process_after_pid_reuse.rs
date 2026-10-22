//! Windows PID ancestry and PID-reuse behavior tests.
//! Kiểm thử ancestry PID Windows và tình huống tái sử dụng PID.

#[test]
fn windows_process_scan_ignores_unrelated_process_after_pid_reuse() {
    use std::ffi::OsString;

    let mut ancestry = super::WindowsProcessAncestry::default();
    let first_snapshot = [
        super::WindowsProcessSnapshot {
            pid: 100,
            parent_pid: None,
            creation_time: Some(1),
            image_name: "runner.exe".to_owned(),
            command: vec![OsString::from("runner.exe")],
        },
        super::WindowsProcessSnapshot {
            pid: 101,
            parent_pid: Some(100),
            creation_time: Some(2),
            image_name: "node.exe".to_owned(),
            command: vec![OsString::from("node.exe"), OsString::from("script.js")],
        },
    ];
    super::scan_windows_process_snapshot(100, &mut ancestry, &first_snapshot, &[], |_| Ok(None))
        .expect("initial ancestry scan should succeed");

    let second_snapshot = [
        super::WindowsProcessSnapshot {
            pid: 100,
            parent_pid: None,
            creation_time: Some(1),
            image_name: "runner.exe".to_owned(),
            command: vec![OsString::from("runner.exe")],
        },
        super::WindowsProcessSnapshot {
            pid: 101,
            parent_pid: Some(999),
            creation_time: Some(10),
            image_name: "npm.exe".to_owned(),
            command: vec![OsString::from("npm.exe"), OsString::from("install")],
        },
    ];

    assert!(matches!(
        super::scan_windows_process_snapshot(100, &mut ancestry, &second_snapshot, &[], |_| Ok(
            None
        ),)
        .expect("unrelated PID reuse without descendants should remain clean"),
        super::WindowsProcessScan::Clean
    ));
}
