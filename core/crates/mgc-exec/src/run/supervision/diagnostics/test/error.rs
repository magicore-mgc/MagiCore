//! Diagnostic error owner module registration marker.
//! Marker đăng ký module error chẩn đoán.

use super::*;

#[test]
fn audit_outcome_records_each_state() {
    assert!(matches!(
        NativeAuditOutcome::Recorded,
        NativeAuditOutcome::Recorded
    ));
    assert!(matches!(
        NativeAuditOutcome::Unavailable,
        NativeAuditOutcome::Unavailable
    ));
    let error = NativeAuditOutcome::WriteFailed(anyhow::Error::msg("write failed"));
    assert!(matches!(error, NativeAuditOutcome::WriteFailed(_)));
}

#[test]
fn device_control_child_diagnostic_does_not_call_the_tool_a_package_manager() {
    let mut error = ForbiddenChildError {
        process_id: 42,
        name: "adb".to_owned(),
        image_name: "adb.exe".to_owned(),
        match_source: "device-control scope",
        matched_token: None,
        matched_token_index: None,
        matched_operation: "device-control invocation".to_owned(),
        status: "exit status: 1".to_owned(),
        audit: NativeAuditOutcome::Unavailable,
        output_tail: String::new(),
    };

    let rendered = error.to_string();
    assert!(rendered.contains("device-control tool 'adb' spawned"));
    assert!(!rendered.contains("package manager"));

    for fallback_source in ["Windows job scan", "process-table scan"] {
        error.match_source = fallback_source;
        assert!(
            error
                .to_string()
                .contains("device-control tool 'adb' spawned"),
            "Windows fallback {fallback_source:?} retains the canonical device-tool name"
        );
    }

    error.name = "npm".to_owned();
    error.match_source = "runtime entrypoint";
    assert!(
        error
            .to_string()
            .contains("forbidden package manager 'npm' spawned")
    );
}
