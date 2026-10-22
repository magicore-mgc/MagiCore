//! Windows PID ancestry and PID-reuse behavior tests.
//! Kiểm thử ancestry PID Windows và tình huống tái sử dụng PID.

#[test]
fn windows_process_scan_keeps_ancestry_across_clean_snapshots() {
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
    assert!(matches!(
        super::scan_windows_process_snapshot(100, &mut ancestry, &first_snapshot, &[], |_| {
            Ok(None)
        })
        .expect("initial tree scan should succeed"),
        super::WindowsProcessScan::Clean
    ));

    let second_snapshot = [
        super::WindowsProcessSnapshot {
            pid: 100,
            parent_pid: None,
            creation_time: Some(1),
            image_name: "runner.exe".to_owned(),
            command: vec![OsString::from("runner.exe")],
        },
        super::WindowsProcessSnapshot {
            pid: 102,
            parent_pid: Some(101),
            creation_time: Some(3),
            image_name: "npm.exe".to_owned(),
            command: vec![OsString::from("npm.exe"), OsString::from("install")],
        },
    ];
    let result =
        super::scan_windows_process_snapshot(100, &mut ancestry, &second_snapshot, &[], |_| {
            Ok(None)
        })
        .expect("detached descendant scan should succeed");

    let super::WindowsProcessScan::Forbidden(found) = result else {
        panic!("npm below a previously observed launcher must still be rejected");
    };
    assert_eq!(found.pid, 102);
    assert_eq!(found.name, "npm");
}
