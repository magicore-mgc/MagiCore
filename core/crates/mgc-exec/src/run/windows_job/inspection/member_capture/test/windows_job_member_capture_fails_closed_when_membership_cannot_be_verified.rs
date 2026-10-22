#[test]
fn windows_job_member_capture_fails_closed_when_membership_cannot_be_verified()
-> Result<(), Box<dyn std::error::Error>> {
    let error = match super::capture_windows_job_members(100, &[100, 101], |_| {
        Err::<(), _>(anyhow::anyhow!(
            "PID from the Job snapshot no longer belongs to the Job"
        ))
    }) {
        Ok(_) => panic!("an unverifiable Job Object PID must fail closed"),
        Err(error) => error,
    };
    assert!(error.to_string().contains("no longer belongs to the Job"));
    Ok(())
}
