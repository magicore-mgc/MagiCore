#[test]
fn windows_job_snapshot_fails_closed_when_membership_never_stabilizes()
-> Result<(), Box<dyn std::error::Error>> {
    let mut next_pid = 102;
    let error = match super::capture_windows_job_process_snapshots(
        100,
        &[100, 101],
        |_process_id| Ok(super::WindowsJobMemberCapture::<()>::Exited),
        |_process_id, _identity| unreachable!(),
        || {
            let pid = next_pid;
            next_pid += 1;
            Ok(vec![pid])
        },
    ) {
        Ok(_) => panic!("a continuously changing Job must fail closed"),
        Err(error) => error,
    };

    assert!(error.to_string().contains("membership kept changing"));
    Ok(())
}
