//! Package-manager child policy tests.
//! Test package-manager child policy.

use super::super::test_support::tmp_dir;
use mgc_exec::prelude::*;
use std::fs;
#[cfg(unix)]
use std::time::Duration;

#[test]
#[cfg(unix)]
fn clean_env_allows_only_the_executor_npm_version_probe_shim() {
    use std::os::unix::fs::PermissionsExt;

    let _fixture = tmp_dir();
    let dir = _fixture.path().join("npm-version-probe");
    fs::create_dir_all(&dir).unwrap();
    let fake_cargo = dir.join("cargo");
    let fake_npm = dir.join("npm");
    let marker = dir.join("external-npm-was-run");
    // Build the command indirectly so static script inspection cannot see it;
    // this exercises the runtime process guard and its read-only probe shim.
    // Dựng tên lệnh gián tiếp để đi qua guard runtime; kiểm tra shim chỉ đọc.
    fs::write(
        &fake_cargo,
        "#!/bin/sh\npm=n\npm=\"${pm}pm\"\n\"$pm\" --version\n",
    )
    .unwrap();
    fs::write(
        &fake_npm,
        format!("#!/bin/sh\nprintf ran > '{}'\nexit 99\n", marker.display()),
    )
    .unwrap();
    for path in [&fake_cargo, &fake_npm] {
        let mut permissions = fs::metadata(path).unwrap().permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(path, permissions).unwrap();
    }

    let opts = ExecOptions {
        clean_env: true,
        env: vec![("PATH".to_string(), dir.display().to_string())],
        timeout: Some(Duration::from_secs(2)),
        capture_full_stdout: true,
        ..Default::default()
    };
    let report = run(fake_cargo.to_str().unwrap(), &[], &opts)
        .expect("npm --version probe should use the executor's inert shim");

    assert_eq!(report.exit_code, 0);
    assert_eq!(report.stdout_full.trim(), "0.0.0");
    assert!(!marker.exists(), "the real npm executable must never run");
}

#[test]
#[cfg(unix)]
fn clean_env_allows_shell_wrapped_npm_version_probe() {
    use std::os::unix::fs::PermissionsExt;

    let _fixture = tmp_dir();
    let dir = _fixture.path().join("npm-version-shell-probe");
    fs::create_dir_all(&dir).unwrap();
    let fake_cargo = dir.join("cargo");
    let fake_npm = dir.join("npm");
    let marker = dir.join("external-npm-was-run");
    // Angular invokes this probe via Node's shell-backed child_process API.
    // Angular gọi probe qua API child_process của Node có bọc shell.
    fs::write(
        &fake_cargo,
        "#!/bin/sh\npm=n\npm=\"${pm}pm\"\n/bin/sh -c \"$pm --version\"\n",
    )
    .unwrap();
    fs::write(
        &fake_npm,
        format!("#!/bin/sh\nprintf ran > '{}'\nexit 99\n", marker.display()),
    )
    .unwrap();
    for path in [&fake_cargo, &fake_npm] {
        let mut permissions = fs::metadata(path).unwrap().permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(path, permissions).unwrap();
    }

    let opts = ExecOptions {
        clean_env: true,
        env: vec![("PATH".to_string(), dir.display().to_string())],
        timeout: Some(Duration::from_secs(2)),
        capture_full_stdout: true,
        ..Default::default()
    };
    let report = run(fake_cargo.to_str().unwrap(), &[], &opts)
        .expect("the exact shell-wrapped npm version probe should use the inert shim");

    assert_eq!(report.exit_code, 0);
    assert_eq!(report.stdout_full.trim(), "0.0.0");
    assert!(!marker.exists(), "the real npm executable must never run");
}

#[test]
#[cfg(unix)]
fn clean_env_still_blocks_shell_wrapped_npm_install() {
    use std::os::unix::fs::PermissionsExt;

    let _fixture = tmp_dir();
    let dir = _fixture.path().join("npm-install-shell-blocked");
    fs::create_dir_all(&dir).unwrap();
    let fake_cargo = dir.join("cargo");
    let fake_npm = dir.join("npm");
    let marker = dir.join("external-npm-was-run");
    // The exact version probe is the only shell wrapper exception.
    // Ngoại lệ shell chỉ áp dụng đúng probe version.
    fs::write(
        &fake_cargo,
        "#!/bin/sh\npm=n\npm=\"${pm}pm\"\n/bin/sh -c \"$pm install\"\n",
    )
    .unwrap();
    fs::write(
        &fake_npm,
        format!("#!/bin/sh\nprintf ran > '{}'\n", marker.display()),
    )
    .unwrap();
    for path in [&fake_cargo, &fake_npm] {
        let mut permissions = fs::metadata(path).unwrap().permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(path, permissions).unwrap();
    }

    let opts = ExecOptions {
        clean_env: true,
        env: vec![("PATH".to_string(), dir.display().to_string())],
        timeout: Some(Duration::from_secs(2)),
        ..Default::default()
    };
    let error = run(fake_cargo.to_str().unwrap(), &[], &opts)
        .expect_err("shell-wrapped npm install must remain blocked");

    assert!(
        error.to_string().contains("forbidden package manager"),
        "unexpected deny error: {error:#}"
    );
    assert!(!marker.exists(), "the real npm executable must never run");
}

#[test]
#[cfg(unix)]
fn runtime_monitor_ignores_package_manager_words_in_shell_code() {
    use std::os::unix::fs::PermissionsExt;

    let _fixture = tmp_dir();
    let dir = _fixture.path().join("opaque-shell-code");
    fs::create_dir_all(&dir).unwrap();
    let fake_cargo = dir.join("cargo");
    fs::write(
        &fake_cargo,
        r##"#!/bin/sh
/bin/sh -c 'printf "%s\n" "npm install; pip install example"'
"##,
    )
    .unwrap();
    let mut permissions = fs::metadata(&fake_cargo).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&fake_cargo, permissions).unwrap();

    let opts = ExecOptions {
        clean_env: true,
        cwd: Some(dir.clone()),
        env: vec![("PATH".to_string(), dir.display().to_string())],
        timeout: Some(Duration::from_secs(2)),
        capture_full_stdout: true,
        ..Default::default()
    };
    let report = run(fake_cargo.to_str().unwrap(), &[], &opts)
        .expect("ordinary shell code that mentions managers must remain runnable");

    assert_eq!(report.exit_code, 0);
    assert!(
        report
            .stdout_full
            .contains("npm install; pip install example")
    );
}

#[test]
#[cfg(unix)]
fn runtime_monitor_blocks_dependency_mutation_from_a_script_child() {
    use std::os::unix::fs::PermissionsExt;

    let _fixture = tmp_dir();
    let dir = _fixture.path().join("descendant-dependency-mutation");
    fs::create_dir_all(&dir).unwrap();
    let fake_node = dir.join("node");
    let fake_cargo = dir.join("cargo");
    let marker = dir.join("cargo-command-ran");
    let audit_log = dir.join(".magicore/exec.log");
    fs::write(&fake_node, "#!/bin/sh\ncargo fetch\n").unwrap();
    fs::write(
        &fake_cargo,
        format!(
            "#!/bin/sh\n/bin/sleep 1\nprintf reached > '{}'\n",
            marker.display()
        ),
    )
    .unwrap();
    for path in [&fake_node, &fake_cargo] {
        let mut permissions = fs::metadata(path).unwrap().permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(path, permissions).unwrap();
    }

    let opts = ExecOptions {
        clean_env: true,
        cwd: Some(dir.clone()),
        env: vec![("PATH".to_string(), dir.display().to_string())],
        timeout: Some(Duration::from_secs(4)),
        log_path: Some(audit_log.clone()),
        ..Default::default()
    };
    let error = run(fake_node.to_str().unwrap(), &[], &opts)
        .expect_err("a descendant cargo fetch must be blocked before it runs");

    assert!(
        error
            .to_string()
            .contains("forbidden package manager 'cargo'"),
        "unexpected runtime mutation refusal: {error:#}"
    );
    assert!(
        !marker.exists(),
        "the descendant package mutation command must be killed before its side effect"
    );
    let audit = fs::read_to_string(&audit_log).expect("native denial must be written to audit");
    assert!(
        audit.contains("native_dependency_mutation_denied"),
        "{audit}"
    );
    assert!(audit.contains("\"child_name\":\"cargo\""), "{audit}");
    assert!(audit.contains("dependency-mutation invocation"), "{audit}");
}

#[test]
#[cfg(unix)]
fn clean_env_still_blocks_npm_install_from_an_indirect_child() {
    use std::os::unix::fs::PermissionsExt;

    let _fixture = tmp_dir();
    let dir = _fixture.path().join("npm-install-blocked");
    fs::create_dir_all(&dir).unwrap();
    let fake_cargo = dir.join("cargo");
    let fake_npm = dir.join("npm");
    let marker = dir.join("external-npm-was-run");
    fs::write(
        &fake_cargo,
        "#!/bin/sh\npm=n\npm=\"${pm}pm\"\n\"$pm\" install\n",
    )
    .unwrap();
    fs::write(
        &fake_npm,
        format!("#!/bin/sh\nprintf ran > '{}'\n", marker.display()),
    )
    .unwrap();
    for path in [&fake_cargo, &fake_npm] {
        let mut permissions = fs::metadata(path).unwrap().permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(path, permissions).unwrap();
    }

    let opts = ExecOptions {
        clean_env: true,
        env: vec![("PATH".to_string(), dir.display().to_string())],
        timeout: Some(Duration::from_secs(2)),
        ..Default::default()
    };
    let error = run(fake_cargo.to_str().unwrap(), &[], &opts).unwrap_err();

    assert!(
        error
            .to_string()
            .contains("MagiCore blocked forbidden package manager: npm")
            || error
                .to_string()
                .contains("forbidden package manager 'npm' spawned"),
        "unexpected error: {error}"
    );
    assert!(!marker.exists(), "npm install must not execute");
}

#[test]
#[cfg(unix)]
fn clean_env_kills_forbidden_pm_spawned_by_absolute_child_path() {
    use std::os::unix::fs::PermissionsExt;

    let _fixture = tmp_dir();
    let dir = _fixture.path().join("absolute-child-pm-bin");
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let fake_cargo = dir.join("cargo");
    let fake_npm = dir.join("npm");
    // Margin matters: under parallel test load a 2s sleep with a 2s timeout
    // races; 5s sleep with a 10s timeout keeps the kill-detection semantics
    // stable regardless of machine load.
    // Biên thời gian quan trọng: dưới tải test song song, sleep 2s với
    // timeout 2s sẽ đua nhau; sleep 5s + timeout 10s giữ nguyên semantics
    // phát hiện kill, ổn định dù máy có tải.
    fs::write(&fake_npm, "#!/bin/sh\n/bin/sleep 5\n").unwrap();
    fs::write(
        &fake_cargo,
        format!("#!/bin/sh\n\"{}\"\n", fake_npm.display()),
    )
    .unwrap();
    for path in [&fake_cargo, &fake_npm] {
        let mut permissions = fs::metadata(path).unwrap().permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(path, permissions).unwrap();
    }

    let opts = ExecOptions {
        clean_env: true,
        timeout: Some(Duration::from_secs(10)),
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
            .contains("references forbidden package manager 'npm'")
            || err
                .to_string()
                .contains("forbidden package manager 'npm' spawned"),
        "unexpected error: {err}"
    );
}

#[test]
#[cfg(unix)]
fn project_binary_rejects_forbidden_binary_name() {
    use std::os::unix::fs::PermissionsExt;

    let _fixture = tmp_dir();
    let dir = _fixture.path().join("project-bin-forbidden-name");
    fs::create_dir_all(&dir).unwrap();
    let fake_npm = dir.join("npm");
    fs::write(&fake_npm, "#!/bin/sh\nexit 0\n").unwrap();
    let mut permissions = fs::metadata(&fake_npm).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&fake_npm, permissions).unwrap();

    let opts = ExecOptions {
        clean_env: true,
        ..Default::default()
    };
    let err = run_project_binary(&fake_npm, &[], &opts).unwrap_err();
    assert!(err.to_string().contains("forbidden package manager"));
}

#[test]
#[cfg(unix)]
fn project_binary_blocks_script_that_executes_forbidden_pm() {
    use std::os::unix::fs::PermissionsExt;

    let _fixture = tmp_dir();
    let dir = _fixture.path().join("project-bin-script-pm");
    fs::create_dir_all(&dir).unwrap();
    let fake_bin = dir.join("tool");
    fs::write(&fake_bin, "#!/bin/sh\nnpm install\n").unwrap();
    let mut permissions = fs::metadata(&fake_bin).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&fake_bin, permissions).unwrap();

    let opts = ExecOptions {
        clean_env: true,
        ..Default::default()
    };
    let err = run_project_binary(&fake_bin, &[], &opts).unwrap_err();
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
fn inherited_project_binary_blocks_script_that_executes_forbidden_pm() {
    use std::os::unix::fs::PermissionsExt;

    let _fixture = tmp_dir();
    let dir = _fixture.path().join("project-bin-inherited-script-pm");
    fs::create_dir_all(&dir).unwrap();
    let fake_bin = dir.join("tool");
    fs::write(&fake_bin, "#!/bin/sh\nnpm install\n").unwrap();
    let mut permissions = fs::metadata(&fake_bin).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&fake_bin, permissions).unwrap();

    let opts = ExecOptions {
        clean_env: true,
        ..Default::default()
    };
    let err = run_project_binary_inherited(&fake_bin, &[], &opts).unwrap_err();
    assert!(
        // An inherited child may exit through the deny shim before monitoring.
        // Child inherited có thể bị deny shim chặn trước lượt quét monitor.
        err.to_string().contains("exited with code 126")
            || err
                .to_string()
                .contains("MagiCore blocked forbidden package manager: npm")
            || err
                .to_string()
                .contains("forbidden package manager 'npm' spawned"),
        "unexpected error: {err}"
    );
}

#[test]
fn npm_is_blocked_even_inside_react_native_subdir() {
    let fixture = tmp_dir();
    let base = fixture.path().to_path_buf();
    let rn = base.join("react-native");
    fs::create_dir_all(&rn).unwrap();
    fs::write(base.join("mgc.toml"), "[app]\nlanguage = \"multi\"\n").unwrap();
    fs::write(
        rn.join("package.json"),
        "{\"dependencies\": {\"react-native\": \"0.7x\"}}",
    )
    .unwrap();

    let opts = ExecOptions {
        cwd: Some(rn.clone()),
        clean_env: true,
        ..Default::default()
    };
    let err = run_inherited("npm", &["install".to_string()], &opts).unwrap_err();
    assert!(
        err.to_string().contains("is forbidden"),
        "npm inside react-native subdir must stay blocked, got: {err}"
    );

    let outside = ExecOptions {
        clean_env: true,
        ..Default::default()
    };
    let err = run_inherited("npm", &["install".to_string()], &outside).unwrap_err();
    assert!(
        err.to_string().contains("is forbidden"),
        "npm outside react-native subdir must be rejected, got: {err}"
    );

    let _ = fs::remove_dir_all(&base);
}
