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
