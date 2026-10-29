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
    // An empty Cargo.lock has nothing to inspect: reporting `Available` made an
    // unresolved Rust project look verified.
    // (Cargo.lock rỗng không có gì để kiểm: báo `Available` khiến project Rust
    // chưa resolve trông như đã xác minh.)
    assert!(matches!(
        report.scanner_status,
        ScannerStatus::Partial { .. }
    ));
}
