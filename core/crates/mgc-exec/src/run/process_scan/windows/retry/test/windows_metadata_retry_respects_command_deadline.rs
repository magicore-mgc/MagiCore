//! Windows process metadata retry and shutdown behavior tests.
//! Kiểm thử retry metadata process Windows và hành vi dừng.

#[test]
fn windows_metadata_retry_respects_command_deadline() {
    use std::cell::Cell;

    let inspections = Cell::new(0);
    let result = super::retry_windows_process_scan(
        std::time::Instant::now(),
        Some(std::time::Duration::from_millis(1)),
        || Ok(false),
        || {
            inspections.set(inspections.get() + 1);
            std::thread::sleep(std::time::Duration::from_millis(5));
            Ok(super::WindowsProcessScan::MissingCommandLine {
                pid: 101,
                image_name: "node.exe".to_owned(),
            })
        },
    )
    .expect("slow snapshot must stop retries at the command deadline");

    assert!(matches!(result, super::WindowsProcessScan::DeadlineReached));
    assert_eq!(
        inspections.get(),
        1,
        "do not start another scan after timeout"
    );
}
