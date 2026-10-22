#[test]
fn windows_go_policy_uses_only_observed_member_environment()
-> Result<(), Box<dyn std::error::Error>> {
    use std::collections::HashMap;
    use std::ffi::OsString;
    let snapshot = |pid, image: &str, args: &[&str]| super::WindowsProcessSnapshot {
        pid,
        parent_pid: Some(100),
        creation_time: Some(u64::from(pid)),
        image_name: image.into(),
        command: args.iter().map(OsString::from).collect(),
    };
    let processes = [
        snapshot(
            101,
            "go.exe",
            &["go.exe", "run", "-mod=readonly", "main.go"],
        ),
        snapshot(
            102,
            "go.exe",
            &["go.exe", "run", "-mod=readonly", "main.go"],
        ),
        snapshot(103, "go.exe", &["go.exe", "get", "example.com/evil"]),
        snapshot(104, "npm.exe", &["npm.exe", "install"]),
    ];
    let paths = HashMap::new();
    let off = vec![("GOPROXY".into(), "off".into())];
    for allowed_env in [false, true] {
        let mut environments = HashMap::from([
            (
                102,
                vec![("GOPROXY".into(), "https://proxy.example".into())],
            ),
            (103, off.clone()),
            (104, off.clone()),
            // Root environment must never authorize a child.
            // Môi trường root không được cấp quyền cho child thiếu bằng chứng.
            (100, off.clone()),
        ]);
        if allowed_env {
            environments.insert(101, off.clone());
        }
        let inspection = super::inspect_windows_job_processes_with_policy_metadata(
            100,
            &[100, 101, 102, 103, 104],
            &processes,
            &[],
            None,
            &super::WindowsProcessPolicyMetadata {
                executable_paths: &paths,
                go_policy_env: &environments,
            },
            None,
            crate::allowlist::ExecutionScope::Install,
        )?;
        assert_eq!(
            inspection.descendants,
            vec![
                (101, (!allowed_env).then(|| "go".into())),
                (102, Some("go".into())),
                (103, Some("go".into())),
                (104, Some("npm".into())),
            ]
        );
    }
    Ok(())
}
