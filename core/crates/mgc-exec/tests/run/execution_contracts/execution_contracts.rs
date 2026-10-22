//! Execution command contracts tests.
//! Test execution command contracts.

use super::super::test_support::tmp_dir;
use mgc_exec::prelude::*;
use std::fs;
#[cfg(unix)]
use std::time::Duration;

#[test]
fn dry_run_does_not_spawn_and_prints() {
    // cargo --version sẽ chạy thật nếu không dry-run — dry_run=true không spawn
    let opts = ExecOptions {
        dry_run: true,
        log_path: None,
        cwd: None,
        ..Default::default()
    };
    let report = run("cargo", &["--version".to_string()], &opts).unwrap();
    assert!(report.dry_run);
    assert_eq!(report.exit_code, 0);
}

#[test]
fn inherited_dry_run_uses_same_report_contract() {
    let opts = ExecOptions {
        dry_run: true,
        log_path: None,
        cwd: None,
        ..Default::default()
    };
    let report = run_inherited("git", &["--version".to_string()], &opts).unwrap();
    assert!(report.dry_run);
    assert_eq!(report.cmd, "git");
    assert_eq!(report.exit_code, 0);
}

#[test]
fn inherited_run_rejects_forbidden_pm_before_spawn() {
    let opts = ExecOptions {
        clean_env: true,
        ..Default::default()
    };
    let err = run_inherited("pnpm", &["install".to_string()], &opts).unwrap_err();
    assert!(err.to_string().contains("forbidden"), "{err}");
}

#[test]
fn reports_process_tree_guard_capability_truthfully() {
    // Tree kill is available on BOTH supported platform families now:
    // unix process groups + Windows taskkill /T (P0-3, 2026-09-15).
    // (Kill cây có trên CẢ hai họ nền tảng: unix process group +
    // Windows taskkill /T.)
    assert!(process_tree_guard_available());
}

#[test]
fn forbidden_tool_rejected_before_spawn() {
    let opts = ExecOptions {
        dry_run: false,
        log_path: None,
        cwd: None,
        ..Default::default()
    };
    let err = run("npm", &["install".to_string()], &opts).unwrap_err();
    assert!(err.to_string().contains("forbidden"));
}

#[test]
fn legacy_compat_option_never_authorizes_package_manager_or_rival_runtime() {
    let opts = ExecOptions {
        execution_scope: Some(ExecutionScope::DevServer),
        compat_runtime: Some("bun".to_string()),
        log_path: None,
        ..Default::default()
    };
    for tool in ["bun", "deno", "composer", "pub"] {
        let err = run(tool, &["--version".to_string()], &opts)
            .expect_err("legacy compat option must not authorize an external process");
        assert!(
            err.to_string().contains("forbidden"),
            "{tool} must be rejected by executor policy before spawn: {err}"
        );
    }
}

#[test]
fn package_resolution_subcommands_are_rejected_before_spawn() {
    let opts = ExecOptions {
        dry_run: true,
        ..Default::default()
    };
    for (tool, args) in [
        ("cargo", vec!["fetch".to_string()]),
        ("cargo", vec!["+stable".to_string(), "fetch".to_string()]),
        ("cargo", vec!["build".to_string()]),
        (
            "cargo",
            vec![
                "test".to_string(),
                "--".to_string(),
                "--offline".to_string(),
            ],
        ),
        (
            "python3",
            vec!["-m".to_string(), "pip".to_string(), "install".to_string()],
        ),
        (
            "go",
            vec!["get".to_string(), "example.test/pkg".to_string()],
        ),
        (
            "go",
            vec![
                "mod".to_string(),
                "edit".to_string(),
                "-require=example.test/pkg@v1.2.3".to_string(),
            ],
        ),
        (
            "go",
            vec![
                "mod".to_string(),
                "init".to_string(),
                "example.test/project".to_string(),
            ],
        ),
        ("go", vec!["work".to_string(), "sync".to_string()]),
        ("flutter", vec!["pub".to_string(), "get".to_string()]),
        ("dotnet", vec!["restore".to_string()]),
        ("swift", vec!["package".to_string(), "resolve".to_string()]),
    ] {
        let error = run_inherited(tool, &args, &opts).unwrap_err();
        assert!(
            error.to_string().contains("blocked")
                || error.to_string().contains("forbidden")
                || error.to_string().contains("not on the allowlist"),
            "{tool} {args:?} was not rejected clearly: {error}"
        );
    }
}

#[test]
fn unknown_tool_rejected() {
    let opts = ExecOptions::default();
    assert!(run("definitely-not-a-real-tool-xyz", &[], &opts).is_err());
}

#[test]
fn audit_log_written_with_redacted_args() {
    let fixture = tmp_dir();
    let dir = fixture.path().to_path_buf();
    let log = dir.join("exec.log");
    let _ = fs::remove_file(&log);
    let opts = ExecOptions {
        dry_run: true,
        log_path: Some(log.clone()),
        cwd: None,
        ..Default::default()
    };
    run(
        "cargo",
        &["--token=leakme".to_string(), "--version".to_string()],
        &opts,
    )
    .unwrap();
    let content = fs::read_to_string(&log).unwrap();
    assert!(content.contains("cargo"));
    assert!(
        !content.contains("leakme"),
        "secret must never reach the audit log: {content}"
    );
    assert!(content.contains("[REDACTED]"));
    let _ = fs::remove_dir_all(&dir);
}

#[test]
fn temp_fixture_is_removed_when_guard_drops() {
    let path = {
        let fixture = tmp_dir();
        let path = fixture.path().to_path_buf();
        assert!(path.is_dir());
        path
    };

    assert!(
        !path.exists(),
        "temporary runner fixture must be removed after the test scope"
    );
}

#[test]
fn missing_tool_fails_with_clear_error() {
    // tool thuộc allowlist nhưng không tồn tại trên máy — spawn fail rõ ràng
    let opts = ExecOptions::default();
    let err = run("pio", &[], &opts).unwrap_err();
    assert!(err.to_string().contains("spawn") || err.to_string().contains("No such"));
}

#[test]
#[cfg(unix)]
fn command_timeout_kills_hung_tool() {
    // P0-3 (Tech Lead 2026-09-15) — the scenario the old test never covered:
    // the tool spawns a GRANDCHILD that inherits stdout/stderr and keeps them
    // open while it sleeps. That is the case that (a) hung the post-kill
    // drain forever (the reader threads waited for an EOF the grandchild
    // never sent) and (b) on macOS survived the timeout entirely, because the
    // /proc-walk tree kill matched nothing without /proc and only the direct
    // child was signalled.
    //
    // Asserts the three things the Tech Lead asked for:
    //   1. run() returns a "timed out" error (not Ok, not a hang)
    //   2. wall-clock stays under 2s for a 1s timeout
    //   3. the grandchild is GONE after run() returns
    //
    // (Đúng kịch bản Tech Lead nêu: tool sinh GRANDCHILD thừa hưởng pipe và
    // giữ nó trong lúc sleep. Đây là ca (a) treo drain sau kill vô hạn và
    // (b) trên macOS sống sót qua timeout vì walk /proc không khớp gì.
    // Kiểm 3 điều: (1) run() trả lỗi "timed out"; (2) wall-clock < 2s với
    // timeout 1s; (3) grandchild đã chết sau khi run() trả về.)
    use std::os::unix::fs::PermissionsExt;
    use std::time::Instant;

    let _fixture = tmp_dir();
    let dir = _fixture.path().join("timeout-tree");
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    let pid_file = dir.join("grandchild.pid");

    // Fake tool: start a 30s sleeper (the grandchild), publish its PID, then
    // keep the shell alive on `wait`. The sleeper inherits the piped stdout
    // and holds it open for the whole 30s.
    // (Tool giả: chạy sleeper 30s (grandchild), ghi PID ra file, rồi giữ
    // shell sống bằng `wait`. Sleeper thừa hưởng stdout dạng pipe và giữ nó
    // suốt 30s.)
    // Use the allowlisted shell only in the explicit project-test scope; an
    // arbitrary name or the default Install scope would stop before timeout.
    // Chỉ dùng shell đã allowlist trong TestRunner; tên tùy ý hoặc Install mặc định
    // sẽ chặn trước khi test timeout.
    let fake_tool = dir.join("bash");
    fs::write(
        &fake_tool,
        format!(
            "#!/bin/sh\n/bin/sleep 30 & echo $! > {}\nwait\n",
            pid_file.display()
        ),
    )
    .unwrap();
    let mut permissions = fs::metadata(&fake_tool).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&fake_tool, permissions).unwrap();

    let opts = ExecOptions {
        clean_env: true,
        cwd: Some(dir.clone()),
        execution_scope: Some(ExecutionScope::TestRunner),
        timeout: Some(Duration::from_secs(1)),
        ..Default::default()
    };

    let started = Instant::now();
    let err = run(fake_tool.to_str().unwrap(), &[], &opts).unwrap_err();
    let elapsed = started.elapsed();

    assert!(
        err.to_string().contains("timed out"),
        "expected a timeout error, got: {err}"
    );
    assert!(
        elapsed < Duration::from_secs(2),
        "run() took {elapsed:?} for a 1s timeout — the process tree was \
         not reaped promptly (grandchild still holding the pipe?)"
    );

    // Grandchild must be gone: signal 0 only succeeds while the PID lives.
    // Zombies are reaped by init once the whole group is killed, so a short
    // bounded poll (not an open-ended sleep) settles the answer.
    // (Grandchild phải chết: signal 0 chỉ thành công khi PID còn sống.
    // Zombie được init reap sau khi cả group bị kill nên poll có hạn là đủ.)
    let grandchild: u32 = fs::read_to_string(&pid_file)
        .unwrap_or_else(|e| panic!("grandchild PID file missing ({e}) — tool never ran"))
        .trim()
        .parse()
        .expect("grandchild PID file must hold a PID");
    assert!(grandchild > 0);
    let deadline = Instant::now() + Duration::from_secs(2);
    let mut still_alive = true;
    while Instant::now() < deadline {
        // Safety: kill(pid, 0) performs no signal delivery — it is the
        // standard liveness probe and never affects the target.
        // (An toàn: kill(pid, 0) không gửi signal — đây là phép dò sống-chết
        // chuẩn, không tác động process đích.)
        #[allow(unsafe_code)]
        let alive = unsafe { libc::kill(grandchild as i32, 0) } == 0;
        if !alive {
            still_alive = false;
            break;
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    assert!(
        !still_alive,
        "grandchild {grandchild} is STILL alive after the timeout kill — \
         terminate_process_tree did not reach the whole tree"
    );

    let _ = fs::remove_dir_all(&dir);
}
