//! Multi-manifest audit status smoke cases.
//! Các smoke case status audit nhiều manifest.

use crate::audit_tests::tmp;
use mgc_app_adapter::audit::scanner::audit_multi;
use mgc_types::adapter::ScannerStatus;

// These existing assertions check the resulting status class only; they do not
// inspect one scanner result per manifest.
// Các assertion hiện có chỉ kiểm tra lớp status; chưa kiểm tra result từng scanner.

#[tokio::test]
async fn audit_multi_aggregates_every_manifest_not_first_match() {
    let dir = tmp("multi-aggregate");
    // Both manifests are present. Missing native lock evidence must
    // remain unsupported regardless of tools installed on this host.
    // Cả hai manifest cùng có mặt. Thiếu lock evidence native vẫn phải
    // unsupported bất kể máy đang cài công cụ nào.
    std::fs::write(dir.join("pubspec.yaml"), "name: test\n").unwrap();
    std::fs::write(dir.join("build.gradle"), "// empty\n").unwrap();

    let report = audit_multi(&dir).await.unwrap();
    assert!(matches!(
        report.scanner_status,
        ScannerStatus::UnsupportedEcosystem { .. } | ScannerStatus::Partial { .. }
    ));
}

#[tokio::test]
async fn audit_multi_aggregate_names_flutter_gap() {
    // Aggregate behavior is independent of external package-manager tools.
    // Hành vi aggregate này không phụ thuộc package manager ngoài máy.
    let dir = tmp("multi-aggregate-2");
    std::fs::write(dir.join("pubspec.yaml"), "name: test\n").unwrap();
    std::fs::write(dir.join("build.gradle.kts"), "// empty\n").unwrap();

    let report = audit_multi(&dir).await.unwrap();
    assert!(matches!(
        report.scanner_status,
        ScannerStatus::UnsupportedEcosystem { .. } | ScannerStatus::Partial { .. }
    ));
}
