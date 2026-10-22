//! Windows process metadata retry and shutdown behavior tests.
//! Kiểm thử retry metadata process Windows và hành vi dừng.

#[test]
fn windows_metadata_retry_stops_when_child_exits() {
    use std::cell::Cell;

    let running_checks = Cell::new(0);
    let inspections = Cell::new(0);
    let result = super::retry_windows_process_scan(
        std::time::Instant::now(),
        None,
        || {
            let check = running_checks.get() + 1;
            running_checks.set(check);
            Ok(check >= 3)
        },
        || {
            inspections.set(inspections.get() + 1);
            Ok(super::WindowsProcessScan::MissingCommandLine {
                pid: 101,
                image_name: "node.exe".to_owned(),
            })
        },
    )
    .expect("finished child should end retries cleanly");

    assert!(matches!(result, super::WindowsProcessScan::ChildExited));
    assert_eq!(inspections.get(), 1, "do not refresh after the child exits");
}
