#[test]
fn windows_job_scan_ignores_global_processes_outside_job_membership()
-> Result<(), Box<dyn std::error::Error>> {
    use std::ffi::OsString;

    let snapshot =
        |pid, parent_pid, image_name: &str, command: &[&str]| super::WindowsProcessSnapshot {
            pid,
            parent_pid,
            creation_time: Some(pid as u64),
            image_name: image_name.to_owned(),
            command: command.iter().map(OsString::from).collect(),
        };
    let processes = [
        snapshot(100, None, "runner.exe", &["runner.exe"]),
        snapshot(101, Some(100), "npm.exe", &["npm.exe", "install"]),
        snapshot(102, Some(100), "LsaIso.exe", &[]),
    ];

    let scanned =
        super::inspect_windows_job_processes_with_shadow(100, &[100, 101], &processes, &[], None)?;

    assert_eq!(scanned.descendants, [(101, Some("npm".to_owned()))]);
    Ok(())
}
