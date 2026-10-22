//! Session process-tree guard tests.
//! Test session process-tree guard.

#[cfg(unix)]
use super::super::test_support::{
    fake_npm, force_kill_test_process_group, make_executable, require_python3,
    test_process_is_alive,
};
#[cfg(unix)]
use mgc_exec::prelude::*;
#[cfg(unix)]
use std::fs;
#[cfg(unix)]
use std::time::Duration;

#[test]
#[cfg(unix)]
fn inherited_env_monitor_kills_a_session_escaped_forbidden_child()
-> Result<(), Box<dyn std::error::Error>> {
    use std::time::Instant;

    require_python3()?;

    let dir = tempfile::tempdir()?;
    let fake_pm = fake_npm(dir.path());
    let fake_cargo = dir.path().join("cargo");
    let pid_file = dir.path().join("escaped-pid");
    let marker = dir.path().join("escaped-ran");
    fs::write(
        &fake_cargo,
        concat!(
            "#!/bin/sh\npython3 -c 'import os,sys; open(sys.argv[1], \"w\").write(str",
            "(os.getpid())); os.setsid(); target=os.environ[\"MGC_TEST_PM\"]; os.exec",
            "v(target, [target, \"install\"])' \"$MGC_TEST_PID_FILE\" &\n/bin/sleep 5",
            "\nwait\n",
        ),
    )?;
    make_executable(&fake_cargo);

    let started = std::time::Instant::now();
    let result = run_project_binary_inherited(
        &fake_cargo,
        &[],
        &ExecOptions {
            cwd: Some(dir.path().to_path_buf()),
            clean_env: false,
            env: vec![
                ("MGC_TEST_PM".to_string(), fake_pm.display().to_string()),
                (
                    "MGC_TEST_PID_FILE".to_string(),
                    pid_file.display().to_string(),
                ),
                ("MGC_TEST_MARKER".to_string(), marker.display().to_string()),
            ],
            timeout: Some(Duration::from_secs(3)),
            ..Default::default()
        },
    );
    let elapsed = started.elapsed();
    let error = result.expect_err("the forbidden descendant must be rejected");
    let pid = fs::read_to_string(&pid_file)?.trim().parse::<u32>()?;
    let deadline = Instant::now() + Duration::from_secs(2);
    while test_process_is_alive(pid) && Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(20));
    }
    let stopped = !test_process_is_alive(pid);
    if !stopped {
        force_kill_test_process_group(&pid_file);
    }

    assert!(
        error
            .to_string()
            .contains("forbidden package manager 'npm' spawned"),
        "unexpected process-guard error: {error}"
    );
    assert!(
        stopped,
        "escaped child PID {pid} survived the process guard"
    );
    assert!(
        !marker.exists(),
        "the escaped package-manager child ran after the guard returned"
    );
    assert!(
        elapsed < Duration::from_secs(2),
        "the runner did not stop the escaped process promptly: {elapsed:?}"
    );
    Ok(())
}

#[test]
#[cfg(unix)]
fn captured_runner_scans_and_stops_children_after_root_exits()
-> Result<(), Box<dyn std::error::Error>> {
    use std::time::Instant;

    require_python3()?;

    let dir = tempfile::tempdir()?;
    let fake_pm = dir.path().join("npm");
    let fake_cargo = dir.path().join("cargo");
    let pid_file = dir.path().join("delayed-pid");
    let marker = dir.path().join("delayed-ran");
    fs::write(
        &fake_pm,
        "#!/bin/sh\nprintf started > \"$MGC_TEST_MARKER\"\n/bin/sleep 1.2\n",
    )?;
    make_executable(&fake_pm);
    fs::write(
        &fake_cargo,
        concat!(
            "#!/bin/sh\npython3 -c 'import os,sys,time; open(sys.argv[1], \"w\").writ",
            "e(str(os.getpid())); os.setsid(); time.sleep(0.5); target=os.environ[\"M",
            "GC_TEST_PM\"]; os.execv(target, [target, \"install\"])' \"$MGC_TEST_PID_",
            "FILE\" &\ni=0\nwhile [ ! -f \"$MGC_TEST_PID_FILE\" ] && [ \"$i\" -lt 100",
            " ]; do /bin/sleep 0.01; i=$((i + 1)); done\n/bin/sleep 0.1\nexit 0\n",
        ),
    )?;
    make_executable(&fake_cargo);

    let started = Instant::now();
    let result = run_project_binary(
        &fake_cargo,
        &[],
        &ExecOptions {
            cwd: Some(dir.path().to_path_buf()),
            clean_env: false,
            env: vec![
                ("MGC_TEST_PM".to_string(), fake_pm.display().to_string()),
                (
                    "MGC_TEST_PID_FILE".to_string(),
                    pid_file.display().to_string(),
                ),
                ("MGC_TEST_MARKER".to_string(), marker.display().to_string()),
            ],
            timeout: Some(Duration::from_secs(5)),
            ..Default::default()
        },
    );
    let elapsed = started.elapsed();
    let error = result.expect_err("a child must not outlive a successful root command");
    let pid = fs::read_to_string(&pid_file)?.trim().parse::<u32>()?;
    let deadline = Instant::now() + Duration::from_secs(2);
    while test_process_is_alive(pid) && Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(20));
    }
    let stopped = !test_process_is_alive(pid);
    if !stopped {
        force_kill_test_process_group(&pid_file);
    }

    assert!(
        error.to_string().contains("child processes remained")
            || error
                .to_string()
                .contains("forbidden package manager 'npm' spawned"),
        "unexpected root-exit guard error: {error}"
    );
    assert!(
        stopped,
        "background child PID {pid} survived root-exit cleanup"
    );
    assert!(
        !marker.exists(),
        "the delayed package-manager child ran after the root exited"
    );
    assert!(
        elapsed < Duration::from_secs(2),
        "the runner drained beyond its deadline after root exit: {elapsed:?}"
    );
    Ok(())
}

#[test]
#[cfg(unix)]
fn inherited_env_monitor_kills_all_session_escaped_forbidden_siblings()
-> Result<(), Box<dyn std::error::Error>> {
    use std::time::Instant;

    require_python3()?;

    let dir = tempfile::tempdir()?;
    let fake_cargo = dir.path().join("cargo");
    let npm_a_dir = dir.path().join("a");
    let npm_b_dir = dir.path().join("b");
    fs::create_dir_all(&npm_a_dir)?;
    fs::create_dir_all(&npm_b_dir)?;
    let fake_npm_a = npm_a_dir.join("npm");
    let fake_npm_b = npm_b_dir.join("npm");
    let pid_file_a = dir.path().join("escaped-a-pid");
    let pid_file_b = dir.path().join("escaped-b-pid");
    let release_file = dir.path().join("release-escaped-pms");
    let marker_a = dir.path().join("escaped-a-ran");
    let marker_b = dir.path().join("escaped-b-ran");
    for (path, marker) in [(&fake_npm_a, &marker_a), (&fake_npm_b, &marker_b)] {
        fs::write(
            path,
            format!(
                "#!/bin/sh\n/bin/sleep 0.5\nprintf started > '{}'\n/bin/sleep 30\n",
                marker.display()
            ),
        )?;
        make_executable(path);
    }
    fs::write(
        &fake_cargo,
        concat!(
            "#!/bin/sh\npython3 -c 'import os,sys,time; open(sys.argv[1], \"w\").write(str",
            "(os.getpid())); os.setsid(); target=os.environ[\"MGC_TEST_PM_A\"]; release=",
            "os.environ[\"MGC_TEST_RELEASE\"]; exec(\"while not os.path.exists(release): ",
            "time.sleep(0.01)\"); os.execv(target, [target, \"install\"])' ",
            "\"$MGC_TEST_PID_FILE_A\" &\npython3 -c 'import os,sys,time; open(sys.argv[1], ",
            "\"w\").write(str(os.getpid())); os.setsid(); target=os.environ[\"MGC_TEST_PM_B\"]; ",
            "release=os.environ[\"MGC_TEST_RELEASE\"]; exec(\"while not os.path.exists(release): ",
            "time.sleep(0.01)\"); os.execv(target, [target, \"install\"])' ",
            "\"$MGC_TEST_PID_FILE_B\" &\nuntil [ -f \"$MGC_TEST_PID_FILE_A\" ] && ",
            "[ -f \"$MGC_TEST_PID_FILE_B\" ]; do /bin/sleep 0.01; done\n",
            ": > \"$MGC_TEST_RELEASE\"\n/bin/sleep 5\nwait\n",
        ),
    )?;
    make_executable(&fake_cargo);

    let started = Instant::now();
    let result = run_project_binary_inherited(
        &fake_cargo,
        &[],
        &ExecOptions {
            cwd: Some(dir.path().to_path_buf()),
            clean_env: false,
            env: vec![
                (
                    "MGC_TEST_PM_A".to_string(),
                    fake_npm_a.display().to_string(),
                ),
                (
                    "MGC_TEST_PM_B".to_string(),
                    fake_npm_b.display().to_string(),
                ),
                (
                    "MGC_TEST_PID_FILE_A".to_string(),
                    pid_file_a.display().to_string(),
                ),
                (
                    "MGC_TEST_PID_FILE_B".to_string(),
                    pid_file_b.display().to_string(),
                ),
                (
                    "MGC_TEST_RELEASE".to_string(),
                    release_file.display().to_string(),
                ),
            ],
            timeout: Some(Duration::from_secs(3)),
            ..Default::default()
        },
    );
    let elapsed = started.elapsed();
    let error = result.expect_err("session-escaped package managers must be rejected");
    let pids = [&pid_file_a, &pid_file_b].map(|path| {
        fs::read_to_string(path)
            .unwrap()
            .trim()
            .parse::<u32>()
            .unwrap()
    });
    let deadline = Instant::now() + Duration::from_secs(2);
    while pids.iter().any(|pid| test_process_is_alive(*pid)) && Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(20));
    }
    let stopped = pids.iter().all(|pid| !test_process_is_alive(*pid));
    if !stopped {
        force_kill_test_process_group(&pid_file_a);
        force_kill_test_process_group(&pid_file_b);
    }

    assert!(
        error
            .to_string()
            .contains("forbidden package manager 'npm' spawned"),
        "unexpected process-guard error: {error}"
    );
    assert!(
        stopped,
        "one or more escaped sibling processes survived: {pids:?}"
    );
    assert!(!marker_a.exists(), "the first fake package manager ran");
    assert!(!marker_b.exists(), "the second fake package manager ran");
    assert!(
        elapsed < Duration::from_secs(2),
        "the runner did not stop both escaped sessions promptly: {elapsed:?}"
    );
    Ok(())
}

#[test]
#[cfg(unix)]
fn captured_runner_rejects_a_session_escape_when_root_exits_immediately()
-> Result<(), Box<dyn std::error::Error>> {
    require_python3()?;

    let dir = tempfile::tempdir()?;
    let fake_pm = dir.path().join("npm");
    let fake_cargo = dir.path().join("cargo");
    let pid_file = dir.path().join("immediate-exit-pid");
    let session_file = dir.path().join("immediate-exit-session");
    let marker = dir.path().join("immediate-exit-ran");
    fs::write(
        &fake_pm,
        format!(
            "#!/bin/sh\nprintf ran > '{}'\n/bin/sleep 2\n",
            marker.display()
        ),
    )?;
    make_executable(&fake_pm);
    fs::write(
        &fake_cargo,
        concat!(
            "#!/bin/sh\npython3 -c 'import os,sys,time; os.setsid(); open(sys.argv[1]",
            ", \"w\").write(str(os.getpid())); open(os.environ[\"MGC_TEST_SESSION_FIL",
            "E\"], \"w\").write(\"ready\"); time.sleep(0.2); target=os.environ[\"MGC_",
            "TEST_PM\"]; os.execv(target, [target, \"install\"])' \"$MGC_TEST_PID_FIL",
            "E\" >/dev/null 2>&1 &\ni=0\nwhile [ ! -f \"$MGC_TEST_SESSION_FILE\" ] &&",
            " [ \"$i\" -lt 100 ]; do /bin/sleep 0.01; i=$((i + 1)); done\nexit 0\n",
        ),
    )?;
    make_executable(&fake_cargo);

    let result = run_project_binary(
        &fake_cargo,
        &[],
        &ExecOptions {
            cwd: Some(dir.path().to_path_buf()),
            clean_env: false,
            env: vec![
                ("MGC_TEST_PM".to_string(), fake_pm.display().to_string()),
                (
                    "MGC_TEST_PID_FILE".to_string(),
                    pid_file.display().to_string(),
                ),
                (
                    "MGC_TEST_SESSION_FILE".to_string(),
                    session_file.display().to_string(),
                ),
            ],
            timeout: Some(Duration::from_secs(2)),
            ..Default::default()
        },
    );
    let pid = fs::read_to_string(&pid_file)?.trim().parse::<u32>()?;
    let deadline = std::time::Instant::now() + Duration::from_millis(500);
    while !marker.exists() && test_process_is_alive(pid) && std::time::Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(10));
    }
    let escaped_child_survived = test_process_is_alive(pid);
    if escaped_child_survived {
        force_kill_test_process_group(&pid_file);
    }

    assert!(
        result.is_err(),
        "root exit must not hide a detached package-manager child"
    );
    assert!(
        !escaped_child_survived,
        "the detached package-manager child survived root exit"
    );
    assert!(!marker.exists(), "the detached package manager ran");
    Ok(())
}

#[test]
#[cfg(unix)]
fn inherited_monitor_kills_forbidden_pm_and_reaps_term_escape_when_handler_is_ready()
-> Result<(), Box<dyn std::error::Error>> {
    use std::time::Instant;

    require_python3()?;

    let dir = tempfile::tempdir()?;
    let fake_pm = dir.path().join("npm");
    let fake_cargo = dir.path().join("cargo");
    let helper_script = dir.path().join("term-helper.py");
    let helper_pid_file = dir.path().join("term-helper-pid");
    let marker = dir.path().join("npm-started");
    fs::write(
        &helper_script,
        concat!(
            "import os, signal, sys, time\nsignal.signal(signal.SIGTERM, signal.SIG_I",
            "GN)\nos.setsid()\nopen(sys.argv[1], 'w').write(str(os.getpid()))\ntime.s",
            "leep(30)\n",
        ),
    )?;
    fs::write(
        &fake_pm,
        concat!(
            "#!/bin/sh\ntrap 'python3 \"$MGC_TEST_HELPER\" \"$MGC_TEST_PID_FILE\" >/d",
            "ev/null 2>&1 &' TERM\nprintf started > \"$MGC_TEST_MARKER\"\nwhile :; do",
            " /bin/sleep 1; done\n",
        ),
    )?;
    make_executable(&fake_pm);
    fs::write(
        &fake_cargo,
        concat!(
            "#!/bin/sh\n# Let the first clean process scan finish before starting the",
            " signal-handler fixture.\n# Chờ lần quét process sạch đầu tiên xong rồi ",
            "mới chạy fixture xử lý signal.\n/bin/sleep 0.1\n\"$MGC_TEST_PM\" install",
            " &\ni=0\nwhile [ ! -f \"$MGC_TEST_MARKER\" ] && [ \"$i\" -lt 100 ]; do /",
            "bin/sleep 0.01; i=$((i + 1)); done\n/bin/sleep 5\nwait\n",
        ),
    )?;
    make_executable(&fake_cargo);

    let started = Instant::now();
    let result = run_project_binary_inherited(
        &fake_cargo,
        &[],
        &ExecOptions {
            cwd: Some(dir.path().to_path_buf()),
            clean_env: false,
            env: vec![
                ("MGC_TEST_PM".to_string(), fake_pm.display().to_string()),
                (
                    "MGC_TEST_HELPER".to_string(),
                    helper_script.display().to_string(),
                ),
                (
                    "MGC_TEST_PID_FILE".to_string(),
                    helper_pid_file.display().to_string(),
                ),
                ("MGC_TEST_MARKER".to_string(), marker.display().to_string()),
            ],
            timeout: Some(Duration::from_secs(5)),
            ..Default::default()
        },
    );
    let elapsed = started.elapsed();
    let error = result.expect_err("a forbidden child must remain rejected during TERM cleanup");
    assert!(
        error
            .to_string()
            .contains("forbidden package manager 'npm' spawned"),
        "unexpected process-guard error: {error}"
    );
    assert!(
        elapsed < Duration::from_secs(2),
        "the guard exceeded its bounded cleanup window: {elapsed:?}"
    );

    if !helper_pid_file.exists() {
        // The monitor can catch npm before its shell executes the first `trap` line.
        // That is a valid earlier deny; only assert the session-escape branch when
        // the fixture confirms its TERM handler was installed.
        // Monitor có thể bắt npm trước khi shell chạy dòng `trap` đầu tiên.
        // Đây là deny sớm hợp lệ; chỉ kiểm nhánh thoát session khi fixture xác nhận
        // handler TERM đã được cài.
        assert!(
            !marker.exists(),
            "TERM handler became ready without starting its session-escape child"
        );
        return Ok(());
    }

    let pid_deadline = Instant::now() + Duration::from_secs(1);
    while !helper_pid_file.exists() && Instant::now() < pid_deadline {
        std::thread::sleep(Duration::from_millis(10));
    }
    let helper_pid = fs::read_to_string(&helper_pid_file)?
        .trim()
        .parse::<u32>()?;
    let stop_deadline = Instant::now() + Duration::from_secs(1);
    while test_process_is_alive(helper_pid) && Instant::now() < stop_deadline {
        std::thread::sleep(Duration::from_millis(10));
    }
    let stopped = !test_process_is_alive(helper_pid);
    if !stopped {
        force_kill_test_process_group(&helper_pid_file);
    }

    assert!(
        stopped,
        "session escaped helper PID {helper_pid} survived TERM/KILL cleanup"
    );
    Ok(())
}
