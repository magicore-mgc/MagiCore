//! Script execution-policy tests.
//! Test policy thực thi script.
use super::*;

#[cfg(unix)]
#[test]
fn explicit_compat_runner_reaches_executor_from_package_script() {
    use crate::commands::compat::CompatMode;
    use crate::commands::run::run_task_script;
    use std::os::unix::fs::PermissionsExt;

    let dir = tempfile::tempdir().unwrap();
    let bin = dir.path().join("node_modules/.bin");
    std::fs::create_dir_all(&bin).unwrap();
    let npm = bin.join("npm");
    std::fs::write(&npm, "#!/bin/sh\nexit 0\n").unwrap();
    let mut permissions = std::fs::metadata(&npm).unwrap().permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(&npm, permissions).unwrap();
    write(
        dir.path().join("package.json"),
        r#"{"scripts":{"dev":"npm run dev"}}"#,
    );

    run_task_script(
        dir.path(),
        "dev",
        &[],
        &CompatMode::Native,
        Some("npm"),
        "run",
    )
    .unwrap();
}

#[test]
fn native_script_still_rejects_package_manager_without_compat() {
    use crate::commands::compat::CompatMode;
    use crate::commands::run::run_task_script;

    let dir = tempfile::tempdir().unwrap();
    write(
        dir.path().join("package.json"),
        r#"{"scripts":{"dev":"npm run dev"}}"#,
    );
    let err =
        run_task_script(dir.path(), "dev", &[], &CompatMode::Native, None, "run").unwrap_err();
    assert!(
        err.to_string()
            .contains("delegates to forbidden package manager"),
        "got: {err}"
    );
}
