#[test]
fn windows_job_snapshot_scans_a_captured_child_that_exits_after_membership_check()
-> Result<(), Box<dyn std::error::Error>> {
    use std::ffi::OsString;

    let captured = super::capture_windows_job_members(100, &[100, 101], |_| Ok(()))?;
    let captured_pids = captured
        .iter()
        .map(|(process_id, _)| *process_id)
        .collect::<Vec<_>>();
    let process_snapshot = [super::WindowsProcessSnapshot {
        pid: 101,
        parent_pid: Some(100),
        creation_time: Some(101),
        image_name: "npm.exe".to_owned(),
        command: vec![OsString::from("npm.exe"), OsString::from("install")],
    }];

    // Captured identity evidence remains authoritative after the process leaves the job.
    // Bằng chứng danh tính đã chụp vẫn có hiệu lực sau khi process rời Job.
    let inspection = super::inspect_windows_job_processes_with_shadow(
        100,
        &captured_pids,
        &process_snapshot,
        &[],
        None,
    )?;
    assert_eq!(inspection.descendants, [(101, Some("npm".to_owned()))]);
    Ok(())
}
