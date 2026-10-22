#![allow(clippy::unwrap_used)]

use super::*;
use mgc_types::adapter::ScannerStatus;

#[tokio::test]
async fn empty_lock_uses_native_audit_route_and_reports_clean() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(project.path().join("Cargo.lock"), "version = 4\n").unwrap();

    let report = audit_rust(project.path()).await.unwrap();

    assert_eq!(report.packages_audited, 0);
    assert_eq!(report.vulnerability_count, 0);
    assert!(matches!(report.scanner_status, ScannerStatus::Available));
}
