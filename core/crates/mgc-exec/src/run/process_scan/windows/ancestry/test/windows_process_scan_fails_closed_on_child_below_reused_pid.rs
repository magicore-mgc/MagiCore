//! Windows PID ancestry and PID-reuse behavior tests.
//! Kiểm thử ancestry PID Windows và tình huống tái sử dụng PID.

#[test]
fn windows_process_scan_fails_closed_on_child_below_reused_pid() {
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
    let first_snapshot = vec![
        snapshot(100, None, 1, "runner.exe", &["runner.exe"]),
        snapshot(101, Some(100), 2, "node.exe", &["node.exe", "script.js"]),
    ];
    let second_snapshot = vec![
        snapshot(100, None, 1, "runner.exe", &["runner.exe"]),
        snapshot(101, Some(999), 10, "unrelated.exe", &["unrelated.exe"]),
        snapshot(102, Some(101), 11, "npm.exe", &["npm.exe", "install"]),
    ];
    let mut ancestry = super::WindowsProcessAncestry::default();
    assert!(matches!(
        super::scan_windows_process_snapshot(100, &mut ancestry, &first_snapshot, &[], |_| Ok(
            None
        ),)
        .expect("initial ancestry scan should succeed"),
        super::WindowsProcessScan::Clean
    ));

    let result = super::retry_windows_process_scan(
        std::time::Instant::now(),
        None,
        || Ok(false),
        || {
            super::scan_windows_process_snapshot(100, &mut ancestry, &second_snapshot, &[], |_| {
                Ok(None)
            })
        },
    );

    let error = match result {
        Ok(_) => panic!("a child below a reused PID must fail closed"),
        Err(error) => error.to_string(),
    };
    assert!(error.contains("ambiguous after PID reuse"), "{error}");
    assert!(!error.contains("forbidden child process"), "{error}");
}
