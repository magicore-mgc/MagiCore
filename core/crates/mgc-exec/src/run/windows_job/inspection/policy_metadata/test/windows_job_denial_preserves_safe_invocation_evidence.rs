#[test]
fn windows_job_denial_preserves_safe_invocation_evidence() {
    use std::ffi::OsString;
    let process = super::WindowsProcessSnapshot {
        pid: 101,
        parent_pid: Some(100),
        creation_time: Some(101),
        image_name: "npm.exe".to_owned(),
        command: ["npm.exe", "install", "private-package-secret"]
            .into_iter()
            .map(OsString::from)
            .collect(),
    };
    let inspection =
        super::inspect_windows_job_processes_with_shadow(100, &[101], &[process], &[], None)
            .expect("inspect forbidden child");
    let super::WindowsProcessScan::Forbidden(found) =
        super::windows_job_scan_from_inspection(inspection)
    else {
        panic!("expected a forbidden process");
    };
    assert_eq!(found.image_name.as_deref(), Some("npm.exe"));
    assert!(found.matched_operation.contains("dependency-mutation"));
    assert!(!found.matched_operation.contains("private-package-secret"));
}
