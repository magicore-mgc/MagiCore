//! Product-route smoke test for the native Rust OSV scanner.
//! Smoke test luồng sản phẩm của scanner OSV Rust native.
#![allow(clippy::unwrap_used)]

use mgc_audit::scanners::audit_rust;
use mgc_types::adapter::ScannerStatus;

#[tokio::test]
async fn native_audit_reports_empty_lock_clean_without_external_tools() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(project.path().join("Cargo.lock"), "version = 4\n").unwrap();

    let report = audit_rust(project.path()).await.unwrap();

    assert_eq!(report.packages_audited, 0);
    assert_eq!(report.vulnerability_count, 0);
    assert!(matches!(report.scanner_status, ScannerStatus::Available));
}
