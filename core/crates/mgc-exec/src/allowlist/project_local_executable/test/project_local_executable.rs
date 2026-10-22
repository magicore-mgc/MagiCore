use crate::allowlist::*;

#[cfg(unix)]
#[test]
fn explicitly_invoked_project_executable_is_scoped_and_confined() {
    use crate::allowlist::check_tool_with_scope;
    use std::os::unix::fs::PermissionsExt;

    let root = tempfile::tempdir().expect("create executable project root");
    let executable = root.path().join(".mgc-probe");
    std::fs::write(&executable, b"probe").expect("write executable fixture");
    let mut permissions = std::fs::metadata(&executable)
        .expect("read executable metadata")
        .permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(&executable, permissions).expect("mark fixture executable");

    assert!(
        check_tool_with_scope("./.mgc-probe", ExecutionScope::DevServer, Some(root.path())).is_ok()
    );
    assert!(
        check_tool_with_scope(
            "./.mgc-probe",
            ExecutionScope::ScriptRunner,
            Some(root.path())
        )
        .is_ok()
    );
    assert!(
        check_tool_with_scope(".mgc-probe", ExecutionScope::DevServer, Some(root.path())).is_err()
    );
    assert!(
        check_tool_with_scope("./.mgc-probe", ExecutionScope::Install, Some(root.path())).is_err()
    );
    assert!(
        check_tool_with_scope(
            "./.mgc-probe",
            ExecutionScope::DeviceControl,
            Some(root.path())
        )
        .is_err()
    );

    let outside = tempfile::tempdir().expect("create outside executable directory");
    let outside_executable = outside.path().join("probe");
    std::fs::write(&outside_executable, b"probe").expect("write outside executable");
    let mut outside_permissions = std::fs::metadata(&outside_executable)
        .expect("read outside executable metadata")
        .permissions();
    outside_permissions.set_mode(0o755);
    std::fs::set_permissions(&outside_executable, outside_permissions)
        .expect("mark outside fixture executable");

    assert!(
        check_tool_with_scope(
            outside_executable.to_str().expect("outside path is UTF-8"),
            ExecutionScope::DevServer,
            Some(root.path())
        )
        .is_err(),
        "executables outside the project root must stay denied"
    );

    let escaped_link = root.path().join("escaped-probe");
    std::os::unix::fs::symlink(&outside_executable, &escaped_link)
        .expect("create project symlink to outside executable");
    assert!(
        check_tool_with_scope(
            "./escaped-probe",
            ExecutionScope::DevServer,
            Some(root.path())
        )
        .is_err(),
        "a project symlink must not authorize a target outside the root"
    );
}
