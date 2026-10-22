#[test]
fn windows_job_member_without_immediate_metadata_fails_closed()
-> Result<(), Box<dyn std::error::Error>> {
    let error = match super::capture_windows_job_process_snapshots(
        100,
        &[100, 101],
        |process_id| Ok(super::WindowsJobMemberCapture::Captured(process_id)),
        |_process_id, _identity| Ok(super::WindowsJobMemberMetadata::Unavailable),
        || Ok(vec![101]),
    ) {
        Ok(_) => panic!("missing child metadata must fail closed"),
        Err(error) => error,
    };
    assert!(
        error
            .to_string()
            .contains("cannot inspect captured Windows job child PID 101")
    );
    Ok(())
}
