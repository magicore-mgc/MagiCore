#[test]
fn windows_job_scan_fails_closed_for_captured_child_missing_snapshot()
-> Result<(), Box<dyn std::error::Error>> {
    let captured = super::capture_windows_job_members(100, &[100, 101], |_| Ok(()))?;
    let captured_pids = captured
        .iter()
        .map(|(process_id, _)| *process_id)
        .collect::<Vec<_>>();
    let error =
        match super::inspect_windows_job_processes_with_shadow(100, &captured_pids, &[], &[], None)
        {
            Ok(_) => panic!("a captured Job Object member without metadata must fail closed"),
            Err(error) => error,
        };
    assert!(
        error
            .to_string()
            .contains("cannot inspect captured Windows job child PID 101")
    );
    Ok(())
}
