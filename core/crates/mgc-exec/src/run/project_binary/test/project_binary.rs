//! Contract tests for project-binary path validation.
//! Test contract cho validation đường dẫn project binary.

use crate::allowlist::ExecutionScope;
use crate::run::{ExecOptions, run_project_binary};

#[cfg(unix)]
#[test]
fn requested_binary_alias_cannot_bypass_package_manager_or_device_scope_denies() {
    use std::os::unix::fs::symlink;
    let project = tempfile::tempdir().expect("project fixture");
    let target = project.path().join("payload");
    std::fs::write(&target, b"inert dry-run fixture").expect("write target");
    let options = ExecOptions {
        cwd: Some(project.path().to_path_buf()),
        execution_scope: Some(ExecutionScope::ScriptRunner),
        dry_run: true,
        ..ExecOptions::default()
    };
    for (name, diagnostic) in [
        ("npm", "forbidden package manager"),
        ("adb", "only allowed in DeviceControl scope"),
        ("xcrun", "only allowed in DeviceControl scope"),
    ] {
        let alias = project.path().join(name);
        symlink(&target, &alias).expect("link protected alias");
        let error = run_project_binary(&alias, &[], &options)
            .expect_err("requested protected name must remain denied after canonicalization");
        assert!(error.to_string().contains(diagnostic), "{name}: {error:#}");
    }
}

#[test]
fn missing_project_binary_is_rejected_before_spawn() {
    let temp = tempfile::tempdir().expect("create project fixture");
    let missing = temp.path().join("bin/missing-tool");

    let error = run_project_binary(&missing, &[], &ExecOptions::default())
        .expect_err("missing project binary must be rejected");

    assert!(error.to_string().contains("is not executable"));
}

#[test]
fn directory_is_rejected_before_spawn() {
    let temp = tempfile::tempdir().expect("create project fixture");

    let error = run_project_binary(temp.path(), &[], &ExecOptions::default())
        .expect_err("a directory cannot be executed as a project binary");

    assert!(error.to_string().contains("is not a file"));
}

#[test]
fn device_control_binary_is_rejected_before_spawn_outside_device_scope() {
    let temp = tempfile::tempdir().expect("create project fixture");
    let binary = temp.path().join("adb.exe");
    std::fs::write(&binary, b"must not execute").expect("write inert fixture");
    let options = ExecOptions {
        execution_scope: Some(ExecutionScope::BuildRunner),
        ..ExecOptions::default()
    };

    let error = run_project_binary(&binary, &[], &options)
        .expect_err("device-control binary must fail before process creation");

    assert!(
        error
            .to_string()
            .contains("only allowed in DeviceControl scope")
    );
}
