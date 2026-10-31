//! Verify an empty observed-group set cannot create a signal failure.
//! Kiểm tra tập group rỗng không tạo lỗi signal và không nhắm process.

#[cfg(unix)]
#[test]
fn signal_dispatch_with_no_observed_groups_is_a_noop() {
    let mut failures = super::super::TerminationFailures::default();
    super::super::signal_process_group_ids(
        &std::collections::HashSet::new(),
        libc::SIGTERM,
        &mut failures,
    );

    assert!(failures.into_result().is_ok());
}

#[cfg(unix)]
#[test]
fn already_stopped_target_errors_are_not_failures() {
    // A PID that vanished (ESRCH) or an exited child the kernel refuses to signal
    // (EPERM) is already stopped; shutdown must not cascade on it.
    // (PID đã biến mất (ESRCH) hoặc child đã thoát mà kernel từ chối signal (EPERM)
    // đều đã dừng; shutdown không được dồn lỗi vì chúng.)
    for code in [libc::ESRCH, libc::EPERM] {
        let source = std::io::Error::from_raw_os_error(code);
        assert!(
            super::super::target_already_stopped(&source),
            "os error {code} must be treated as an already-stopped target"
        );
    }
    for code in [libc::EINVAL, libc::EIO] {
        let source = std::io::Error::from_raw_os_error(code);
        assert!(
            !super::super::target_already_stopped(&source),
            "os error {code} still means the signal path is broken"
        );
    }
}

#[cfg(unix)]
#[test]
fn signalling_a_gone_group_records_no_failure() {
    // `kill(-pgid, SIGKILL)` on a group that no longer exists returns ESRCH; a sweep
    // over such a target has to succeed so normal shutdown stays quiet.
    // (`kill(-pgid, SIGKILL)` trên group không còn tồn tại trả ESRCH; lượt quét trên
    // đích đó phải thành công để lần tắt bình thường im lặng.)
    let mut failures = super::super::TerminationFailures::default();
    let gone_group = u32::MAX / 2;
    super::super::signal_process_group_ids(
        &std::collections::HashSet::from([gone_group]),
        libc::SIGKILL,
        &mut failures,
    );

    assert!(
        failures.into_result().is_ok(),
        "a gone process group must not be reported as a termination failure"
    );
}
