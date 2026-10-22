// Typed diagnostic construction contracts.
// Contract tạo chẩn đoán typed.

use super::*;

#[test]
fn timeout_error_preserves_budget_status_and_bounded_tail() {
    let error = CommandTimeoutError {
        timeout: Duration::from_millis(250),
        status: "exit status: 9".to_owned(),
        output_tail: "last output".to_owned(),
    };
    let rendered = error.to_string();
    assert!(rendered.contains("timed out after 250ms"));
    assert!(rendered.contains("status after kill: exit status: 9"));
    assert!(rendered.ends_with("last output"));
}

#[test]
fn forbidden_child_error_keeps_safe_match_evidence_and_audit_failure_source() {
    use std::error::Error;
    let error = ForbiddenChildError {
        process_id: 23,
        name: "npm".to_owned(),
        image_name: "npm.exe".to_owned(),
        match_source: "runtime entrypoint",
        matched_token: Some("npm-cli.js".to_owned()),
        matched_token_index: Some(2),
        matched_operation: "dependency-mutation invocation".to_owned(),
        status: "exit status: 1".to_owned(),
        audit: NativeAuditOutcome::WriteFailed(anyhow::Error::msg("audit write denied")),
        output_tail: "sanitized tail".to_owned(),
    };
    let rendered = error.to_string();
    assert!(rendered.contains("npm-cli.js' at argv[2]"));
    assert!(rendered.contains("audit event write failed: audit write denied"));
    assert!(rendered.ends_with("sanitized tail"));
    assert!(error.source().is_some());
    assert!(!rendered.contains("private-package-secret"));
}
