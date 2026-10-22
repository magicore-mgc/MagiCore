// Tests cleanup aggregation and primary error-source preservation.
// Kiểm tra ghép cleanup và giữ source lỗi chính.

use super::{
    EnvironmentError, ExecReport, SpawnError, combine_execution_and_environment_cleanup,
    combine_execution_and_handle_cleanup,
};

fn report() -> ExecReport {
    ExecReport {
        cmd: "native-tool".to_owned(),
        args: Vec::new(),
        exit_code: 0,
        duration_ms: 0,
        dry_run: false,
        stdout_tail: String::new(),
        stderr_tail: String::new(),
        stdout_full: String::new(),
    }
}

#[test]
fn successful_cleanup_preserves_the_execution_report() {
    let windows = combine_execution_and_handle_cleanup(Ok(report()), None)
        .expect("no handle-cleanup failure must preserve success");
    assert_eq!(windows.cmd, "native-tool");

    let environment = combine_execution_and_environment_cleanup(Ok(report()), Ok(()))
        .expect("successful environment cleanup must preserve success");
    assert_eq!(environment.cmd, "native-tool");
}

#[test]
fn successful_cleanup_preserves_an_execution_failure() {
    let windows = combine_execution_and_handle_cleanup(
        Err(anyhow::anyhow!("child execution failed")),
        None,
    )
    .expect_err("execution failure must survive a clean handle cleanup");
    assert_eq!(windows.to_string(), "child execution failed");

    let environment = combine_execution_and_environment_cleanup(
        Err(anyhow::anyhow!("child execution failed")),
        Ok(()),
    )
    .expect_err("execution failure must survive a clean environment cleanup");
    assert_eq!(environment.to_string(), "child execution failed");
}

#[test]
fn cleanup_only_failures_are_returned_without_wrapping() {
    use std::error::Error;

    let windows = combine_execution_and_handle_cleanup(
        Ok(report()),
        Some(anyhow::anyhow!("CloseHandle failed")),
    )
    .expect_err("handle cleanup failure must fail execution");
    assert_eq!(windows.to_string(), "CloseHandle failed");

    let environment = combine_execution_and_environment_cleanup(
        Ok(report()),
        Err(EnvironmentError::RemoveDirectory {
            path: std::path::PathBuf::from("shadow-path"),
            source: std::io::Error::other("cleanup permission denied"),
        }),
    )
    .expect_err("environment cleanup failure must fail execution");
    let environment = environment
        .downcast_ref::<EnvironmentError>()
        .expect("cleanup-only error must retain its typed environment owner");
    assert!(matches!(
        environment,
        EnvironmentError::RemoveDirectory { .. }
    ));
    assert_eq!(
        Error::source(environment)
            .expect("cleanup I/O error must remain the source")
            .to_string(),
        "cleanup permission denied"
    );
}

#[test]
fn windows_handle_cleanup_failure_is_reported_with_execution_failure() {
    use std::error::Error;

    let result = combine_execution_and_handle_cleanup(
        Err(anyhow::anyhow!("child execution failed")),
        Some(anyhow::anyhow!("CloseHandle failed with error code 6")),
    );
    let error = result.expect_err("simultaneous execution and cleanup failures must fail");
    let error = error
        .downcast_ref::<SpawnError>()
        .expect("combined error must retain the spawn owner type");
    assert!(error.to_string().contains("child execution failed"));
    assert!(error.to_string().contains("CloseHandle failed with error code 6"));
    assert_eq!(
        Error::source(error)
            .expect("primary execution error must remain the source")
            .to_string(),
        "child execution failed"
    );
}

#[test]
fn environment_cleanup_failure_keeps_execution_error_as_source() {
    use std::error::Error;

    let result = combine_execution_and_environment_cleanup(
        Err(anyhow::anyhow!("child execution failed")),
        Err(EnvironmentError::RemoveDirectory {
            path: std::path::PathBuf::from("shadow-path"),
            source: std::io::Error::other("cleanup permission denied"),
        }),
    );
    let error = result.expect_err("environment cleanup failure must be reported");
    let error = error
        .downcast_ref::<SpawnError>()
        .expect("combined error must retain the spawn owner type");
    assert!(error.to_string().contains("child execution failed"));
    assert!(error.to_string().contains("cleanup permission denied"));
    assert_eq!(
        Error::source(error)
            .expect("execution error must remain the source")
            .to_string(),
        "child execution failed"
    );
}
