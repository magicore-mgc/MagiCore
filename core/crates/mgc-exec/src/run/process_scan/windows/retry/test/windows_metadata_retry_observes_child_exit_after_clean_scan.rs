//! Windows process metadata retry and shutdown behavior tests.
//! Kiểm thử retry metadata process Windows và hành vi dừng.

#[test]
fn windows_metadata_retry_observes_child_exit_after_clean_scan() {
    use std::cell::Cell;

    let running_checks = Cell::new(0);
    let result = super::retry_windows_process_scan(
        std::time::Instant::now(),
        None,
        || {
            let check = running_checks.get() + 1;
            running_checks.set(check);
            Ok(check == 2)
        },
        || Ok(super::WindowsProcessScan::Clean),
    )
    .expect("child exit during a clean scan should be observed");

    assert!(matches!(result, super::WindowsProcessScan::ChildExited));
}
