// Compatibility audit redaction, scope and deduplication tests.
// Test redaction, phạm vi và chống ghi trùng audit compatibility.

use super::*;
use crate::run::implementation::process_monitor::CompatAuditContext;
use std::collections::HashSet;
use std::ffi::OsString;
use std::sync::Mutex;

#[test]
fn absent_compat_scope_does_not_write_an_audit_event() {
    let root = tempfile::tempdir().expect("create audit fixture");
    let log = root.path().join("compat.log");
    let command = [OsString::from("npm"), OsString::from("install")];

    audit_compat_process(None, 17, "npm", &command)
        .expect("an absent compatibility scope is a no-op");

    assert!(!log.exists());
}

#[test]
fn matching_compat_process_is_redacted_and_recorded_once() {
    let root = tempfile::tempdir().expect("create audit fixture");
    let log = root.path().join("compat.log");
    let context = CompatAuditContext {
        manager: "npm",
        log_path: &log,
        cwd: root.path(),
        seen_processes: Mutex::new(HashSet::new()),
    };
    let command = [
        OsString::from("npm"),
        OsString::from("install"),
        OsString::from("--token=private-value"),
    ];

    audit_compat_process(Some(&context), 42, "npm", &command)
        .expect("matching package-manager child is audit-logged");
    audit_compat_process(Some(&context), 42, "npm", &command)
        .expect("the same process row is deduplicated");

    let contents = std::fs::read_to_string(&log).expect("read compat audit rows");
    assert_eq!(contents.lines().count(), 1);
    assert!(contents.contains("compat_runner_child_observed"));
    assert!(contents.contains("[REDACTED]"));
    assert!(!contents.contains("private-value"));
}

#[test]
fn unrelated_process_does_not_create_compat_audit_file() {
    let root = tempfile::tempdir().expect("create audit fixture");
    let log = root.path().join("compat.log");
    let context = CompatAuditContext {
        manager: "npm",
        log_path: &log,
        cwd: root.path(),
        seen_processes: Mutex::new(HashSet::new()),
    };
    let command = [OsString::from("node"), OsString::from("build.js")];

    audit_compat_process(Some(&context), 43, "node", &command)
        .expect("unmatched process is ignored");

    assert!(!log.exists());
}

#[test]
fn poisoned_dedup_state_fails_with_a_typed_error() {
    let root = tempfile::tempdir().expect("create audit fixture");
    let log = root.path().join("compat.log");
    let context = CompatAuditContext {
        manager: "npm",
        log_path: &log,
        cwd: root.path(),
        seen_processes: Mutex::new(HashSet::new()),
    };
    let poisoned = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _guard = context.seen_processes.lock().expect("dedup state is initially healthy");
        panic!("poison compat dedup state for contract test");
    }));
    assert!(poisoned.is_err());
    let command = [OsString::from("npm"), OsString::from("install")];

    let error = audit_compat_process(Some(&context), 44, "npm", &command)
        .expect_err("poisoned dedup state must fail closed");
    assert!(matches!(error, CompatAuditError::DedupStatePoisoned));
}

#[test]
fn failed_audit_write_keeps_a_typed_error_source() {
    use std::error::Error;

    let root = tempfile::tempdir().expect("create audit fixture");
    let log_directory = root.path().join("compat.log");
    std::fs::create_dir(&log_directory).expect("use a directory where the log file should be");
    let context = CompatAuditContext {
        manager: "npm",
        log_path: &log_directory,
        cwd: root.path(),
        seen_processes: Mutex::new(HashSet::new()),
    };
    let command = [OsString::from("npm"), OsString::from("install")];

    let error = audit_compat_process(Some(&context), 45, "npm", &command)
        .expect_err("writing over a directory must fail closed");
    assert!(matches!(error, CompatAuditError::WriteFailed(_)));
    assert_eq!(error.to_string(), "compat audit event could not be written");
    assert!(error.source().is_some());
}
