#[test]
fn windows_job_scan_fails_closed_for_unreadable_job_members()
-> Result<(), Box<dyn std::error::Error>> {
    use std::ffi::OsString;

    let processes = [
        super::WindowsProcessSnapshot {
            pid: 100,
            parent_pid: None,
            creation_time: Some(100),
            image_name: "runner.exe".to_owned(),
            command: vec![OsString::from("runner.exe")],
        },
        super::WindowsProcessSnapshot {
            pid: 102,
            parent_pid: Some(100),
            creation_time: Some(102),
            image_name: "LsaIso.exe".to_owned(),
            command: Vec::new(),
        },
    ];

    let error =
        super::inspect_windows_job_processes_with_shadow(100, &[100, 102], &processes, &[], None)?;

    assert_eq!(
        error.missing_command_line,
        Some((102, "LsaIso.exe".to_owned()))
    );
    assert!(matches!(
        super::windows_job_scan_from_inspection(error),
        super::WindowsProcessScan::MissingCommandLine { pid: 102, .. }
    ));
    Ok(())
}
