// Dry-run contract for the shared process-spawn pipeline.
// Test contract dry-run của luồng spawn process dùng chung.

use super::{SpawnError, execute_command};
use crate::run::implementation::{ExecOptions, OutputMode};

#[test]
fn dry_run_returns_without_resolving_or_spawning_the_command() {
    let project = tempfile::tempdir().expect("create project directory");
    let options = ExecOptions {
        dry_run: true,
        cwd: Some(project.path().to_path_buf()),
        ..ExecOptions::default()
    };
    let args = vec!["--version".to_owned()];

    let report = execute_command(
        "mgc-command-that-does-not-exist-for-dry-run",
        &args,
        &options,
        OutputMode::Capture,
        None,
        true,
    )
    .expect("dry-run must return before command resolution or process spawn");

    assert!(report.dry_run);
    assert_eq!(report.exit_code, 0);
    assert_eq!(report.duration_ms, 0);
    assert!(report.stdout_tail.is_empty());
    assert!(report.stderr_tail.is_empty());
}

#[test]
fn spawn_failures_keep_typed_source_context() {
    use std::error::Error;
    use std::io;

    let error = SpawnError::CommandSpawnFailed {
        command: "native-tool".to_string(),
        source: io::Error::new(io::ErrorKind::NotFound, "missing executable"),
    };
    assert_eq!(
        error.to_string(),
        "failed to spawn 'native-tool': missing executable"
    );
    assert!(Error::source(&error).is_some());
}

#[test]
fn current_directory_failures_keep_the_io_source() {
    use std::error::Error;
    use std::io;

    let error = SpawnError::ResolveCurrentDirectory(io::Error::new(
        io::ErrorKind::PermissionDenied,
        "current directory unavailable",
    ));
    assert!(
        error
            .to_string()
            .contains("cannot resolve current execution directory")
    );
    let source = Error::source(&error).expect("current-directory source is retained");
    assert_eq!(source.to_string(), "current directory unavailable");
}

