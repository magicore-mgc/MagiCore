//! Verdict precedence cases — test thứ tự ưu tiên verdict.
use crate::aggregate::report_collection::collect_reports;
use crate::aggregate::status_resolution::resolve_status;
use mgc_types::adapter::{AuditReport, ScannerStatus};

#[test]
fn failure_wins_over_missing_tool() {
    let reports = vec![
        (
            "node:failed".to_owned(),
            AuditReport::scanner_failed("scanner", "failed"),
        ),
        (
            "java:missing".to_owned(),
            AuditReport::tool_missing("gradle", "install gradle"),
        ),
    ];
    let mut summary = collect_reports(&reports);

    match resolve_status(&reports, &mut summary) {
        ScannerStatus::Failed { scanner, reason } => {
            assert_eq!(scanner, "scanner");
            assert_eq!(reason, "failed");
        }
        other => panic!("expected Failed, got {other:?}"),
    }
}

#[test]
fn empty_input_resolves_to_available() {
    let reports: Vec<(String, AuditReport)> = Vec::new();
    let mut summary = collect_reports(&reports);

    assert!(matches!(
        resolve_status(&reports, &mut summary),
        ScannerStatus::Available
    ));
}

#[test]
fn available_plus_unsupported_resolves_to_partial() {
    let reports = vec![
        ("node:osv".to_owned(), AuditReport::clean(1)),
        (
            "flutter:none".to_owned(),
            AuditReport::unsupported_ecosystem("flutter"),
        ),
    ];
    let mut summary = collect_reports(&reports);

    match resolve_status(&reports, &mut summary) {
        ScannerStatus::Partial {
            scanned,
            skipped,
            reasons,
        } => {
            assert_eq!(scanned, 1);
            assert_eq!(skipped, 0);
            assert!(reasons.iter().any(|reason| reason.contains("flutter")));
        }
        other => panic!("expected Partial, got {other:?}"),
    }
}
