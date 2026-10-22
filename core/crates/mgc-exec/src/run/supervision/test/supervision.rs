// Real child-process timeout contract for the supervisor.
// Contract timeout với process con thật cho supervisor.

#[cfg(unix)]
#[test]
fn timeout_terminates_a_real_project_binary_through_mgc() {
    use crate::run::{ExecOptions, run_project_binary};
    use std::path::Path;
    use std::time::Duration;

    let project = tempfile::tempdir().expect("create project directory");
    let sleep = Path::new("/bin/sleep");
    assert!(sleep.is_file(), "Unix CI must provide /bin/sleep");
    let options = ExecOptions {
        cwd: Some(project.path().to_path_buf()),
        timeout: Some(Duration::from_millis(100)),
        ..ExecOptions::default()
    };

    let error = run_project_binary(sleep, &["5".to_owned()], &options)
        .expect_err("long-running child must be terminated at the configured timeout");

    assert!(error.to_string().contains("command timed out after"));
}

#[test]
fn supervision_errors_keep_the_original_io_source() {
    use std::error::Error;

    let error = super::error::SupervisionError::KillChild {
        action: "enforcing the execution timeout",
        source: std::io::Error::new(std::io::ErrorKind::PermissionDenied, "kill denied"),
    };

    assert!(
        error
            .to_string()
            .contains("enforcing the execution timeout")
    );
    assert_eq!(
        Error::source(&error)
            .expect("child kill source is retained")
            .to_string(),
        "kill denied"
    );
}

#[cfg(unix)]
#[test]
fn process_inspection_errors_keep_the_failure_source() {
    use std::error::Error;

    let error = super::error::SupervisionError::ProcessInspection {
        phase: "during command execution",
        status: std::os::unix::process::ExitStatusExt::from_raw(0),
        source: anyhow::anyhow!("process metadata unavailable"),
    };

    assert!(error.to_string().contains("terminated fail-closed"));
    assert_eq!(
        Error::source(&error)
            .expect("process inspection source is retained")
            .to_string(),
        "process metadata unavailable"
    );
}
