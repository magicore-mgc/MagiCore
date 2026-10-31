//! Regression: task scripts observe the caller environment.
//! Hồi quy: task script thấy được environment của caller.

use super::*;

/// Variable the fixture script prints back to the test.
/// Biến mà script fixture in ra cho test.
const PROBE_ENV_KEY: &str = "MGC_TEST_RUN_ENV_PROBE";
const PROBE_ENV_VALUE: &str = "inherited-by-mgc-run";

/// Serializes process-wide environment writes across the run tests.
/// Tuần tự hóa ghi environment toàn cục process giữa các test của run.
static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[test]
fn run_task_script_inherits_caller_environment() {
    use crate::commands::compat::CompatMode;
    use crate::commands::run::run_task_script;

    let _guard = ENV_LOCK.lock().unwrap_or_else(|error| error.into_inner());
    let dir = tempfile::tempdir().expect("temporary project");
    // The fixture writes its own observation file; redefining the script in shell
    // would need control operators that the script gate correctly rejects.
    // (Fixture tự ghi file quan sát; viết lại script kiểu shell sẽ cần toán tử
    // điều khiển mà cổng script chặn đúng.)
    let observed = dir.path().join("env-probe.txt");
    let fixture = dir.path().join("bash");
    std::fs::write(
        &fixture,
        format!("#!/bin/sh\nprintf '%s' \"${{{PROBE_ENV_KEY}}}\" > \"$1\"\n"),
    )
    .expect("write probe fixture");
    make_executable(&fixture);
    write(
        dir.path().join("mgc.toml"),
        &format!(
            "[scripts]\nprobe = \"{} {}\"\n",
            fixture.display(),
            observed.display()
        ),
    );
    // SAFETY: the test process mutates its own environment under the test lock.
    // AN TOÀN: tiến trình test chỉ đổi environment của chính nó dưới khóa test.
    #[allow(unsafe_code)]
    unsafe {
        std::env::set_var(PROBE_ENV_KEY, PROBE_ENV_VALUE);
    }

    let result = run_task_script(dir.path(), "probe", &[], &CompatMode::Native, None, "run");

    // SAFETY: the test process mutates its own environment under the test lock.
    // AN TOÀN: tiến trình test chỉ đổi environment của chính nó dưới khóa test.
    #[allow(unsafe_code)]
    unsafe {
        std::env::remove_var(PROBE_ENV_KEY);
    }
    result.expect("the probe script must run");
    assert_eq!(
        std::fs::read_to_string(&observed).expect("probe script writes its observation"),
        PROBE_ENV_VALUE,
        "PORT/NODE_ENV-style caller variables must reach `mgc run` scripts"
    );
}

/// Mark the fixture executable on Unix.
/// Đánh dấu fixture thực thi trên Unix.
fn make_executable(path: &std::path::Path) {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        let mut permissions = std::fs::metadata(path)
            .expect("fixture metadata")
            .permissions();
        permissions.set_mode(0o755);
        std::fs::set_permissions(path, permissions).expect("fixture permissions");
    }
    #[cfg(not(unix))]
    {
        let _ = path;
    }
}
