#[test]
fn windows_job_device_control_children_follow_root_scope() {
    use std::collections::HashMap;
    use std::ffi::OsString;

    let processes = [
        super::WindowsProcessSnapshot {
            pid: 101,
            parent_pid: Some(100),
            creation_time: Some(101),
            image_name: "adb.exe".to_owned(),
            command: ["adb.exe", "devices"]
                .into_iter()
                .map(OsString::from)
                .collect(),
        },
        super::WindowsProcessSnapshot {
            pid: 102,
            parent_pid: Some(100),
            creation_time: Some(102),
            image_name: "xcrun".to_owned(),
            command: ["xcrun", "simctl"].into_iter().map(OsString::from).collect(),
        },
    ];
    let paths = HashMap::new();
    let environments = HashMap::new();
    let policy_metadata = super::WindowsProcessPolicyMetadata {
        executable_paths: &paths,
        go_policy_env: &environments,
    };

    let denied = super::inspect_windows_job_processes_with_policy_metadata(
        100,
        &[100, 101, 102],
        &processes,
        &[],
        None,
        &policy_metadata,
        None,
        crate::allowlist::ExecutionScope::BuildRunner,
    )
    .expect("inspect Windows Job child policy");
    assert_eq!(
        denied.descendants,
        vec![(101, Some("adb".into())), (102, Some("xcrun".into()))]
    );
    assert_eq!(
        denied
            .first_forbidden
            .as_ref()
            .map(|process| process.name.as_str()),
        Some("adb")
    );

    let allowed = super::inspect_windows_job_processes_with_policy_metadata(
        100,
        &[100, 101, 102],
        &processes,
        &[],
        None,
        &policy_metadata,
        None,
        crate::allowlist::ExecutionScope::DeviceControl,
    )
    .expect("inspect authorized DeviceControl children");
    assert_eq!(allowed.descendants, vec![(101, None), (102, None)]);
    assert!(allowed.first_forbidden.is_none());
}
