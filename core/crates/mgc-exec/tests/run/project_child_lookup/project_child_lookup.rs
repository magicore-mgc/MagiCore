//! Project child-command lookup guard tests.
//! Test project child-command lookup guard.

#[cfg(unix)]
use super::super::test_support::tmp_dir;
#[cfg(unix)]
use mgc_exec::prelude::*;
#[cfg(unix)]
use std::fs;
#[cfg(unix)]
use std::time::Duration;

#[test]
#[cfg(unix)]
fn clean_env_blocks_forbidden_pm_spawned_by_child_path_lookup() {
    use std::os::unix::fs::PermissionsExt;

    let _fixture = tmp_dir();
    let dir = _fixture.path().join("child-pm-bin");
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let fake_cargo = dir.join("cargo");
    fs::write(&fake_cargo, "#!/bin/sh\nnpm install\n").unwrap();
    let mut permissions = fs::metadata(&fake_cargo).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&fake_cargo, permissions).unwrap();

    let opts = ExecOptions {
        clean_env: true,
        env: vec![("PATH".to_string(), dir.display().to_string())],
        timeout: Some(Duration::from_secs(2)),
        ..Default::default()
    };

    let err = run(
        fake_cargo.to_str().unwrap(),
        &["--version".to_string()],
        &opts,
    )
    .unwrap_err();
    assert!(
        err.to_string()
            .contains("MagiCore blocked forbidden package manager: npm")
            || err
                .to_string()
                .contains("forbidden package manager 'npm' spawned"),
        "unexpected error: {err}"
    );
}

#[test]
#[cfg(unix)]
fn inherited_env_still_blocks_forbidden_pm_spawned_by_project_tool() {
    use std::os::unix::fs::PermissionsExt;

    let _fixture = tmp_dir();
    let dir = _fixture.path().join("inherited-child-pm-bin");
    fs::create_dir_all(&dir).unwrap();
    let fake_cargo = dir.join("cargo");
    let fake_npm = dir.join("npm");
    let marker = dir.join("npm-was-run");
    fs::write(
        &fake_npm,
        format!("#!/bin/sh\nprintf ran > '{}'\n", marker.display()),
    )
    .unwrap();
    fs::write(&fake_cargo, "#!/bin/sh\nnpm install\n").unwrap();
    for path in [&fake_cargo, &fake_npm] {
        let mut permissions = fs::metadata(path).unwrap().permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(path, permissions).unwrap();
    }

    let opts = ExecOptions {
        cwd: Some(dir.clone()),
        clean_env: false,
        env: vec![("PATH".to_string(), dir.display().to_string())],
        timeout: Some(Duration::from_secs(2)),
        ..Default::default()
    };

    let result = run_project_binary_inherited(&fake_cargo, &[], &opts);
    assert!(
        result.is_err(),
        "MagiCore must reject a forbidden child process"
    );
    assert!(
        !marker.exists(),
        "the package-manager child must not execute before detection"
    );
}
