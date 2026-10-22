use crate::parallel::test::support::{python, task};
use crate::parallel::{ParallelOptions, run_parallel};
use crate::run::ExecOptions;

// All-success returns every result in task order.
#[test]
fn all_tasks_success() {
    let opts = ExecOptions::default();
    let popts = ParallelOptions {
        kill_others: true,
        prefix_output: false,
    };
    let results = run_parallel(
        &[
            task(python(), &["-c", "print('one')"]),
            task(python(), &["-c", "print('two')"]),
        ],
        &opts,
        &popts,
    )
    .unwrap();
    assert_eq!(results.len(), 2);
    assert!(results.iter().all(|result| result.exit_code == 0));
    assert!(results[0].stdout.contains("one"));
    assert!(results[1].stdout.contains("two"));
}

// Concurrent task audit records remain one valid JSON object per line.
#[test]
fn parallel_audit_log_keeps_records_separate() {
    let dir = tempfile::TempDir::new().unwrap();
    let audit_path = dir.path().join("exec.jsonl");
    let opts = ExecOptions {
        log_path: Some(audit_path.clone()),
        ..Default::default()
    };
    let tasks: Vec<_> = (0..12)
        .map(|index| {
            let script = format!("print('task-{index}')");
            task(python(), &["-c", script.as_str()])
        })
        .collect();

    let results = run_parallel(
        &tasks,
        &opts,
        &ParallelOptions {
            kill_others: true,
            prefix_output: false,
        },
    )
    .unwrap();

    assert_eq!(results.len(), tasks.len());
    let log = std::fs::read_to_string(audit_path).unwrap();
    let lines: Vec<_> = log.lines().collect();
    assert_eq!(lines.len(), tasks.len());
    for line in lines {
        let entry: serde_json::Value = serde_json::from_str(line).unwrap();
        assert_eq!(entry["cmd"], "python3");
    }
}

// First failure is reported with its task index.
#[test]
fn first_failure_reported() {
    let opts = ExecOptions::default();
    let popts = ParallelOptions {
        kill_others: false,
        prefix_output: false,
    };
    let error = run_parallel(
        &[
            task(python(), &["-c", "print('ok')"]),
            task(python(), &["-c", "import sys; sys.exit(3)"]),
        ],
        &opts,
        &popts,
    )
    .unwrap_err();
    assert!(
        error
            .to_string()
            .contains("parallel task 1 failed with exit code 3"),
        "unexpected error: {error}"
    );
}

// kill-others stops a slow sibling: a 60s sleeper must not run 60s.
#[test]
fn kill_others_stops_slow_sibling() {
    let opts = ExecOptions::default();
    let popts = ParallelOptions {
        kill_others: true,
        prefix_output: false,
    };
    let started = std::time::Instant::now();
    let error = run_parallel(
        &[
            task(python(), &["-c", "import time; time.sleep(60)"]),
            task(python(), &["-c", "import sys; sys.exit(1)"]),
        ],
        &opts,
        &popts,
    )
    .unwrap_err();
    let elapsed = started.elapsed();
    assert!(
        error.to_string().contains("parallel task 1 failed"),
        "unexpected error: {error}"
    );
    assert!(
        elapsed < std::time::Duration::from_secs(30),
        "slow sibling was not stopped: {elapsed:?}"
    );
}

// Forbidden tools never reach spawn, even in parallel.
#[test]
fn parallel_rejects_forbidden_tools() {
    let opts = ExecOptions::default();
    let popts = ParallelOptions::default();
    let error = run_parallel(&[task("npm", &["install", "x"])], &opts, &popts).unwrap_err();
    assert!(
        error.to_string().contains("forbidden"),
        "unexpected error: {error}"
    );
}

// A parallel task uses the shared descendant monitor, not only direct preflight.
#[cfg(unix)]
#[test]
fn parallel_rejects_forbidden_package_manager_descendant() {
    use std::os::unix::fs::PermissionsExt;

    let dir = tempfile::TempDir::new().unwrap();
    let package_manager = dir.path().join("npm");
    std::fs::write(&package_manager, "#!/bin/sh\nsleep 30\n").unwrap();
    std::fs::set_permissions(&package_manager, std::fs::Permissions::from_mode(0o700)).unwrap();

    let opts = ExecOptions {
        disable_timeout: true,
        env: vec![(
            "MGC_TEST_PM_PATH".to_string(),
            package_manager.to_string_lossy().into_owned(),
        )],
        ..Default::default()
    };
    let started = std::time::Instant::now();
    let error = run_parallel(
        &[task(
            python(),
            &[
                "-c",
                concat!(
                    "import os, subprocess, time; ",
                    "subprocess.Popen([os.environ['MGC_TEST_PM_PATH']]); ",
                    "time.sleep(30)"
                ),
            ],
        )],
        &opts,
        &ParallelOptions {
            kill_others: true,
            prefix_output: false,
        },
    )
    .unwrap_err();

    assert!(
        error
            .to_string()
            .contains("forbidden package manager 'npm' spawned"),
        "unexpected error: {error}"
    );
    assert!(
        started.elapsed() < std::time::Duration::from_secs(20),
        "forbidden descendant was not terminated promptly: {:?}",
        started.elapsed()
    );
}
