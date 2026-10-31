//! Shutdown tolerance when the monitored root is already gone.
//! Dung nạp khi tắt process lúc root được giám sát đã mất.

#[cfg(unix)]
use super::super::test_support::make_executable;
#[cfg(unix)]
use mgc_exec::prelude::*;
#[cfg(unix)]
use std::time::{Duration, Instant};

/// Upper bound for a shutdown that has nothing left to signal.
/// Trần thời gian cho lần tắt không còn gì để gửi signal.
#[cfg(unix)]
const IDLE_SHUTDOWN_BUDGET: Duration = Duration::from_secs(2);

#[test]
#[cfg(unix)]
fn ending_an_already_exited_root_succeeds_without_a_process_table_error()
-> Result<(), Box<dyn std::error::Error>> {
    // `mgc dev` used to print `TermSweep: process table did not contain
    // monitored root PID ... failed to send SIGKILL ... Operation not permitted`
    // on every normal shutdown. The tree is empty, so the run must simply finish.
    // (`mgc dev` từng in lỗi TermSweep/SIGKILL EPERM mỗi lần tắt bình thường. Cây
    // đã rỗng nên lượt chạy chỉ việc kết thúc bình thường.)
    let project = tempfile::tempdir()?;
    // A project-local `bash` fixture stays inside the allowlisted, reviewed tool
    // set while still exiting immediately. (Fixture `bash` trong project nằm trong
    // allowlist đã duyệt và vẫn thoát ngay.)
    let short_lived = project.path().join("bash");
    std::fs::write(&short_lived, "#!/bin/sh\nexit 0\n")?;
    make_executable(&short_lived);
    let options = ExecOptions {
        cwd: Some(project.path().to_path_buf()),
        execution_scope: Some(ExecutionScope::TestRunner),
        disable_timeout: true,
        ..ExecOptions::default()
    };
    let started = Instant::now();

    let report = run_inherited(
        short_lived.to_str().expect("utf-8 fixture path"),
        &[],
        &options,
    )?;

    assert_eq!(report.exit_code, 0);
    assert!(
        started.elapsed() < IDLE_SHUTDOWN_BUDGET,
        "a clean shutdown waited on a gone tree for {:?}",
        started.elapsed()
    );
    Ok(())
}
