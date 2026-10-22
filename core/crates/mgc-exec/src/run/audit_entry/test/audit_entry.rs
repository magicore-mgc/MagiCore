use super::super::{ExecReport, entry_from};
use std::path::Path;

#[test]
fn report_fields_are_preserved_and_sensitive_arguments_are_redacted() {
    let report = ExecReport {
        cmd: "mgc build".to_string(),
        args: vec!["--token=private-value".to_string()],
        exit_code: 7,
        duration_ms: 42,
        dry_run: true,
        stdout_tail: String::new(),
        stderr_tail: "failed".to_string(),
        stdout_full: String::new(),
    };

    let entry = entry_from(&report, Path::new("/workspace/project"));

    assert_eq!(entry.cmd, "mgc build");
    assert_eq!(entry.args, vec!["--token=[REDACTED]".to_string()]);
    assert_eq!(entry.cwd, "/workspace/project");
    assert_eq!(entry.exit_code, 7);
    assert_eq!(entry.duration_ms, 42);
    assert!(entry.dry_run);
    assert!(!entry.args[0].contains("private-value"));
}
