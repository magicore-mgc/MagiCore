//! Preflight security and dry-run boundary tests.
//! Test ranh giới bảo mật preflight và dry-run.

use super::super::super::super::error::SpawnError;
use super::super::{complete, prepare};
use crate::run::implementation::ExecOptions;

#[test]
fn path_traversal_is_rejected_before_dry_run_completion() {
    let project = tempfile::tempdir().expect("create project directory");
    let options = ExecOptions {
        dry_run: true,
        cwd: Some(project.path().to_path_buf()),
        ..ExecOptions::default()
    };
    let args = vec!["../outside-project".to_owned()];

    assert!(prepare("tool", &args, &options, None).is_err());
}

#[test]
fn compat_runner_requires_audit_log_before_dry_run() {
    let project = tempfile::tempdir().expect("create project directory");
    let options = ExecOptions {
        dry_run: true,
        cwd: Some(project.path().to_path_buf()),
        compat_runner: Some("npm".to_owned()),
        ..ExecOptions::default()
    };
    let error = prepare("npm", &[], &options, None)
        .err()
        .expect("compat mode must fail closed without its audit destination");

    assert!(matches!(
        error.downcast_ref::<SpawnError>(),
        Some(SpawnError::CompatRunnerRequiresAuditLog)
    ));
}

#[test]
fn dry_run_completion_writes_redacted_audit_without_live_setup() {
    let project = tempfile::tempdir().expect("create project directory");
    let audit = project.path().join("exec.log");
    let options = ExecOptions {
        dry_run: true,
        cwd: Some(project.path().to_path_buf()),
        log_path: Some(audit.clone()),
        ..ExecOptions::default()
    };
    let args = vec!["--token=secret-value".to_owned()];
    let prepared = prepare("tool", &args, &options, None).expect("prepare valid dry-run");
    let mut prepared_args = prepared.safe_args;
    let report = complete(
        "tool",
        &mut prepared_args,
        &prepared.cwd,
        true,
        options.log_path.as_deref(),
    )
    .expect("write dry-run audit")
    .expect("dry-run report");

    assert!(report.dry_run);
    assert!(!report.args.iter().any(|arg| arg.contains("secret-value")));
    let audit = std::fs::read_to_string(audit).expect("read dry-run audit row");
    assert!(!audit.contains("secret-value"));
    assert!(audit.contains("\"dry_run\":true"));
}

#[test]
fn dry_run_completion_without_log_still_returns_report() {
    let project = tempfile::tempdir().expect("create project directory");
    let options = ExecOptions {
        dry_run: true,
        cwd: Some(project.path().to_path_buf()),
        ..ExecOptions::default()
    };
    let prepared = prepare("tool", &[], &options, None).expect("prepare valid dry-run");
    let mut prepared_args = prepared.safe_args;
    let report = complete("tool", &mut prepared_args, &prepared.cwd, true, None)
        .expect("complete without audit output")
        .expect("dry-run report");

    assert_eq!(report.exit_code, 0);
    assert_eq!(report.duration_ms, 0);
}

#[test]
fn non_dry_run_completion_leaves_report_construction_to_spawn_owner() {
    let project = tempfile::tempdir().expect("create project directory");
    let options = ExecOptions {
        cwd: Some(project.path().to_path_buf()),
        ..ExecOptions::default()
    };
    let prepared = prepare("tool", &[], &options, None).expect("prepare valid invocation");

    let mut prepared_args = prepared.safe_args;
    assert!(
        complete("tool", &mut prepared_args, &prepared.cwd, false, None)
            .expect("continue live execution")
            .is_none()
    );
}
