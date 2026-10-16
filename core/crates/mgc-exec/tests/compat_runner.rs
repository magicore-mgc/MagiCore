//! Compat-runner tests — kiểm thử compat-runner.
//!
//! `--compat-runner <manager>` (Spec rev4 §7/§9): the declared runner's own
//! invocation skips the native name/verb guards and runs under the compat
//! audit-only monitor; every other tool keeps native-deny.
//! (`--compat-runner <manager>` (Spec rev4 §7/§9): invocation của runner đã
//! khai báo bỏ qua guard tên/verb native và chạy dưới monitor audit-only
//! compat; mọi tool khác giữ native-deny.)

#![cfg(test)]
#![allow(clippy::unwrap_used)]

use mgc_exec::allowlist::{QUALIFIED_COMPAT_RUNNERS, compat_runner_allows, parse_compat_runner};
use mgc_exec::prelude::{ExecOptions, run, run_project_binary_inherited};

// Only JS-ecosystem managers have a compat contract today.
// (Hiện chỉ manager JS có compat contract.)
#[test]
fn qualified_managers_parse() {
    for manager in ["npm", "pnpm", "yarn", "bun"] {
        assert!(QUALIFIED_COMPAT_RUNNERS.contains(&manager));
        assert_eq!(
            parse_compat_runner(Some(manager)).unwrap(),
            Some(manager.to_string())
        );
    }
    assert_eq!(parse_compat_runner(None).unwrap(), None);
}

// Normalization: paths, case and Windows suffixes collapse to basename.
// (Chuẩn hóa: path, hoa/thường và hậu tố Windows gộp về basename.)
#[test]
fn parse_normalizes_manager_name() {
    assert_eq!(
        parse_compat_runner(Some("/usr/local/bin/NPM")).unwrap(),
        Some("npm".to_string())
    );
    assert_eq!(
        parse_compat_runner(Some("Yarn.CMD")).unwrap(),
        Some("yarn".to_string())
    );
}

// Managers without a compat contract stay rejected with a clear error.
// (Manager chưa có compat contract vẫn bị từ chối rõ ràng.)
#[test]
fn unqualified_managers_rejected() {
    for manager in ["pip", "uv", "cargo", "go", "pub", "composer", "deno"] {
        let err = parse_compat_runner(Some(manager)).unwrap_err();
        assert!(
            err.to_string().contains("not a qualified compat manager"),
            "unexpected error for {manager}: {err}"
        );
    }
}

// Basename matching for the declared runner itself.
// (Khớp basename cho đúng runner đã khai báo.)
#[test]
fn allows_declared_runner_basename() {
    assert!(compat_runner_allows("npm", Some("npm")));
    assert!(compat_runner_allows(
        "/x/node_modules/.bin/npm",
        Some("npm")
    ));
    assert!(!compat_runner_allows("pip", Some("npm")));
    assert!(!compat_runner_allows("npm", None));
    assert!(!compat_runner_allows("npm", Some("yarn")));
}

// Without compat, forbidden tools are denied before any spawn.
// (Không compat, tool cấm bị deny trước khi spawn — không cần binary thật.)
#[test]
fn native_still_denies_forbidden_tools() {
    let opts = ExecOptions::default();
    let err = run("npm", &["install".to_string(), "x".to_string()], &opts).unwrap_err();
    assert!(
        err.to_string().contains("forbidden"),
        "unexpected error: {err}"
    );
}

// A declared compat runner does not exempt other tools.
// (Runner đã khai báo không miễn trừ cho tool khác.)
#[test]
fn compat_does_not_exempt_other_tools() {
    let opts = ExecOptions {
        compat_runner: Some("npm".to_string()),
        ..Default::default()
    };
    let err = run("pip", &["install".to_string(), "x".to_string()], &opts).unwrap_err();
    assert!(
        err.to_string().contains("forbidden"),
        "unexpected error: {err}"
    );
}

// The declared runner itself passes the pre-spawn guards: spawn a fake
// `npm` executable and prove it runs instead of being denied.
// (Runner đã khai báo qua được guard trước spawn: dựng executable `npm` giả
// và chứng minh nó chạy thay vì bị deny.)
#[test]
#[cfg(unix)]
fn declared_runner_spawns_fake_binary() {
    use std::os::unix::fs::PermissionsExt;

    let dir = tempfile::TempDir::new().unwrap();
    let fake = dir.path().join("npm");
    std::fs::write(&fake, "#!/bin/sh\nexit 0\n").unwrap();
    let mut permissions = std::fs::metadata(&fake).unwrap().permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(&fake, permissions).unwrap();

    let opts = ExecOptions {
        compat_runner: Some("npm".to_string()),
        cwd: Some(dir.path().to_path_buf()),
        log_path: Some(dir.path().join("exec.log")),
        ..Default::default()
    };
    let args = vec![
        "install".to_string(),
        "--token".to_string(),
        "secret-value".to_string(),
    ];
    let report = run(fake.to_str().unwrap(), &args, &opts).unwrap();
    assert_eq!(report.exit_code, 0);
    let rows = std::fs::read_to_string(dir.path().join("exec.log")).unwrap();
    let scope: serde_json::Value = serde_json::from_str(rows.lines().next().unwrap()).unwrap();
    assert_eq!(scope["event"], "compat_runner_scope");
    assert_eq!(scope["manager"], "npm");
    assert_eq!(scope["root_cmd"], fake.to_string_lossy().as_ref());
    assert_eq!(scope["scope"], "root_process_tree");
    assert!(scope["root_args"].to_string().contains("[REDACTED]"));
    assert!(!rows.contains("secret-value"));
}

// A compat subtree cannot start without a durable audit destination.
// (Không mở compat subtree nếu thiếu nơi ghi audit bền vững.)
#[test]
#[cfg(unix)]
fn compat_runner_requires_audit_path_before_spawn() {
    use std::os::unix::fs::PermissionsExt;

    let dir = tempfile::TempDir::new().unwrap();
    let marker = dir.path().join("spawned");
    let fake = dir.path().join("npm");
    std::fs::write(
        &fake,
        format!("#!/bin/sh\nprintf spawned > '{}'\n", marker.display()),
    )
    .unwrap();
    let mut permissions = std::fs::metadata(&fake).unwrap().permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(&fake, permissions).unwrap();

    let opts = ExecOptions {
        compat_runner: Some("npm".to_string()),
        cwd: Some(dir.path().to_path_buf()),
        ..Default::default()
    };
    let error = run(fake.to_str().unwrap(), &[], &opts).unwrap_err();
    assert!(error.to_string().contains("requires an audit log path"));
    assert!(
        !marker.exists(),
        "compat runner spawned without audit logging"
    );
}

// A package manager observed below the selected root is recorded by the tree monitor.
// (Process manager được monitor thấy dưới command gốc phải được ghi vào audit.)
#[test]
#[cfg(unix)]
fn compat_runner_audits_matching_child_process() {
    use std::os::unix::fs::PermissionsExt;

    let dir = tempfile::TempDir::new().unwrap();
    let bin = dir.path().join("bin");
    std::fs::create_dir_all(&bin).unwrap();
    let fake = bin.join("npm");
    let marker = dir.path().join("npm-child-ran");
    std::fs::write(
        &fake,
        format!(
            "#!/bin/sh\nprintf spawned > '{}'\n/bin/sleep 0.35\nexit 0\n",
            marker.display()
        ),
    )
    .unwrap();
    let mut permissions = std::fs::metadata(&fake).unwrap().permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(&fake, permissions).unwrap();
    let parent_script = dir.path().join("parent.sh");
    std::fs::write(
        &parent_script,
        "#!/bin/sh\nnpm install --token child-secret\n/usr/bin/true\n",
    )
    .unwrap();
    let mut permissions = std::fs::metadata(&parent_script).unwrap().permissions();
    permissions.set_mode(0o755);
    std::fs::set_permissions(&parent_script, permissions).unwrap();

    let opts = ExecOptions {
        compat_runner: Some("npm".to_string()),
        cwd: Some(dir.path().to_path_buf()),
        log_path: Some(dir.path().join("exec.log")),
        env: vec![("PATH".to_string(), bin.to_string_lossy().into_owned())],
        ..Default::default()
    };
    run_project_binary_inherited(
        std::path::Path::new("/bin/sh"),
        &[parent_script.to_string_lossy().into_owned()],
        &opts,
    )
    .unwrap();

    let log = std::fs::read_to_string(dir.path().join("exec.log")).unwrap();
    let records = log
        .lines()
        .map(|line| serde_json::from_str::<serde_json::Value>(line).unwrap())
        .collect::<Vec<_>>();
    assert!(marker.exists(), "fake npm did not spawn; audit log:\n{log}");
    let child = records
        .iter()
        .find(|record| record["event"] == "compat_runner_child_observed")
        .unwrap_or_else(|| panic!("package-manager child was not audited; log:\n{log}"));
    assert_eq!(child["manager"], "npm");
    assert!(child["args"].to_string().contains("[REDACTED]"));
    assert!(!log.contains("child-secret"));
}
