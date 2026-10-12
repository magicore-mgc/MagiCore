//! Shutdown semantics tests — kiểm thử ngữ nghĩa dừng.
//!
//! A shutdown request (Spec rev4 §16.3 — decided) terminates the running
//! tree and fails the wait instead of orphaning children; without a request
//! execution is unaffected.
//! (Yêu cầu dừng (Spec rev4 §16.3 — đã chốt) terminate cây đang chạy và fail
//! vòng chờ thay vì bỏ orphan; không có yêu cầu thì thực thi bình thường.)

#![cfg(test)]
#![allow(clippy::unwrap_used)]

use mgc_exec::prelude::{ExecOptions, clear_shutdown, request_shutdown, run};

static SHUTDOWN_TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

// Single sequential test: the shutdown flag is process-global, so the
// no-request and pending-request phases must not race each other.
// (Một test tuần tự duy nhất: cờ shutdown toàn process, nên hai pha không
// yêu cầu và đang chờ không được đua nhau.)
#[test]
fn shutdown_semantics() {
    let _shutdown_guard = SHUTDOWN_TEST_LOCK
        .lock()
        .expect("acquire process-global shutdown test lock");
    // Phase 1: without a request, a quick command runs normally.
    // (Pha 1: không có yêu cầu, lệnh nhanh chạy bình thường.)
    clear_shutdown();
    let opts = ExecOptions::default();
    let report = run(
        "python3",
        &["-c".to_string(), "print('alive')".to_string()],
        &opts,
    )
    .unwrap();
    assert_eq!(report.exit_code, 0);
    assert!(report.stdout_tail.contains("alive"));

    // Phase 2: a pending request fails a running command fast instead of
    // waiting it out.
    // (Pha 2: yêu cầu đang chờ fail lệnh đang chạy nhanh thay vì chờ hết.)
    request_shutdown();
    let opts = ExecOptions {
        timeout: Some(std::time::Duration::from_secs(60)),
        ..Default::default()
    };
    let started = std::time::Instant::now();
    let err = run(
        "python3",
        &["-c".to_string(), "import time; time.sleep(60)".to_string()],
        &opts,
    )
    .unwrap_err();
    clear_shutdown();
    assert!(
        err.to_string().contains("interrupted by shutdown request"),
        "unexpected error: {err}"
    );
    assert!(
        started.elapsed() < std::time::Duration::from_secs(30),
        "shutdown did not stop the tree promptly"
    );
}

#[cfg(unix)]
#[test]
fn supervisor_cancel_terminates_the_owned_process_tree() {
    let _shutdown_guard = SHUTDOWN_TEST_LOCK
        .lock()
        .expect("acquire process-global shutdown test lock");
    let cancel_flag = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let worker_flag = std::sync::Arc::clone(&cancel_flag);
    let worker = std::thread::spawn(move || {
        let opts = ExecOptions {
            timeout: Some(std::time::Duration::from_secs(60)),
            ..Default::default()
        };
        mgc_exec::prelude::run_inherited_cancellable(
            "python3",
            &["-c".to_string(), "import time; time.sleep(60)".to_string()],
            &opts,
            &worker_flag,
        )
    });

    std::thread::sleep(std::time::Duration::from_millis(150));
    cancel_flag.store(true, std::sync::atomic::Ordering::SeqCst);
    let started = std::time::Instant::now();
    let error = worker.join().unwrap().unwrap_err();

    assert!(
        error.to_string().contains("cancelled by its supervisor"),
        "unexpected error: {error:#}"
    );
    assert!(
        started.elapsed() < std::time::Duration::from_secs(10),
        "supervisor cancellation did not terminate the process promptly"
    );
}

// Windows cancellation must terminate descendants through the owned Job Object.
// Trên Windows, cancel phải dừng descendants qua Job Object được sở hữu.
#[cfg(windows)]
#[test]
fn supervisor_cancel_terminates_windows_descendants() {
    let _shutdown_guard = SHUTDOWN_TEST_LOCK
        .lock()
        .expect("acquire process-global shutdown test lock");
    use std::sync::Arc;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::time::{Duration, Instant};
    use sysinfo::{Pid, ProcessRefreshKind, ProcessesToUpdate, System};

    let dir = tempfile::TempDir::new().unwrap();
    let child_pid_path = dir.path().join("child.pid");
    let cancel_flag = Arc::new(AtomicBool::new(false));
    let worker_flag = Arc::clone(&cancel_flag);
    let worker_path = child_pid_path.clone();
    let worker = std::thread::spawn(move || {
        let mut opts = ExecOptions {
            timeout: Some(Duration::from_secs(60)),
            ..Default::default()
        };
        opts.env.push((
            "MGC_TEST_CHILD_PID_FILE".to_string(),
            worker_path.to_string_lossy().into_owned(),
        ));
        mgc_exec::prelude::run_inherited_cancellable(
            "python3",
            &[
                "-c".to_string(),
                "import os, subprocess, time; child = subprocess.Popen(['python3', '-c', 'import time; time.sleep(60)']); open(os.environ['MGC_TEST_CHILD_PID_FILE'], 'w').write(str(child.pid)); time.sleep(60)".to_string(),
            ],
            &opts,
            &worker_flag,
        )
    });

    let child_deadline = Instant::now() + Duration::from_secs(10);
    while !child_pid_path.exists() && Instant::now() < child_deadline {
        std::thread::sleep(Duration::from_millis(20));
    }
    if !child_pid_path.exists() {
        cancel_flag.store(true, Ordering::SeqCst);
        let _ = worker.join();
        panic!("test child did not start");
    }
    let child_pid_text = match std::fs::read_to_string(&child_pid_path) {
        Ok(text) => text,
        Err(error) => {
            cancel_flag.store(true, Ordering::SeqCst);
            let _ = worker.join();
            panic!("failed to read test child PID: {error}");
        }
    };
    let child_pid = match child_pid_text.trim().parse::<u32>() {
        Ok(pid) => pid,
        Err(error) => {
            cancel_flag.store(true, Ordering::SeqCst);
            let _ = worker.join();
            panic!("invalid test child PID: {error}");
        }
    };

    cancel_flag.store(true, Ordering::SeqCst);
    let error = worker.join().unwrap().unwrap_err();
    assert!(
        error.to_string().contains("cancelled by its supervisor"),
        "unexpected error: {error:#}"
    );

    let child_pid = Pid::from_u32(child_pid);
    let stop_deadline = Instant::now() + Duration::from_secs(5);
    let mut system = System::new();
    loop {
        system.refresh_processes_specifics(
            ProcessesToUpdate::All,
            true,
            ProcessRefreshKind::nothing(),
        );
        if !system.processes().contains_key(&child_pid) {
            break;
        }
        assert!(
            Instant::now() < stop_deadline,
            "Windows descendant survived supervisor cancellation"
        );
        std::thread::sleep(Duration::from_millis(50));
    }
}
