//! Cross-owner Windows process scan integration tests.
//! Kiểm thử tích hợp scanner Windows qua nhiều owner.

#[test]
fn windows_process_scan_retries_transient_creation_time_unavailability() {
    use std::cell::Cell;
    use std::ffi::OsString;

    let snapshot = [
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
            creation_time: None,
            image_name: "node.exe".to_owned(),
            command: vec![OsString::from("node.exe"), OsString::from("script.js")],
        },
    ];
    let mut ancestry = super::WindowsProcessAncestry::default();
    let inspections = Cell::new(0);
    let identity_queries = Cell::new(0);

    let result = super::retry_windows_process_scan(
        std::time::Instant::now(),
        None,
        || Ok(false),
        || {
            inspections.set(inspections.get() + 1);
            super::scan_windows_process_snapshot(100, &mut ancestry, &snapshot, &[], |_| {
                let query = identity_queries.get() + 1;
                identity_queries.set(query);
                Ok((query > 1).then_some(2))
            })
        },
    )
    .expect("transient process creation-time lookup should recover");

    assert!(matches!(result, super::WindowsProcessScan::Clean));
    assert_eq!(
        inspections.get(),
        2,
        "retry must refresh the full process tree"
    );
    assert_eq!(
        identity_queries.get(),
        2,
        "retry must query process identity again"
    );
}
