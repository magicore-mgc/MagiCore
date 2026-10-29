//! Product-route smoke test for the native Rust OSV scanner.
//! Smoke test luồng sản phẩm của scanner OSV Rust native.
#![allow(clippy::unwrap_used)]

use mgc_audit::scanners::audit_rust;
use mgc_types::adapter::ScannerStatus;

#[tokio::test]
async fn native_audit_reports_empty_lock_unverified_without_external_tools() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(project.path().join("Cargo.lock"), "version = 4\n").unwrap();

    let report = audit_rust(project.path()).await.unwrap();

    assert_eq!(report.packages_audited, 0);
    assert_eq!(report.vulnerability_count, 0);
    // The test name promised "clean" for an EMPTY lock, i.e. verified safety for
    // a graph with nothing in it. It now asserts the honest state.
    // (Tên test hứa "clean" cho lock RỖNG — tức an toàn đã xác minh cho graph
    // không có gì. Nay khẳng định trạng thái trung thực.)
    assert!(matches!(
        report.scanner_status,
        ScannerStatus::Partial { .. }
    ));
    assert!(!report.is_clean());
}
