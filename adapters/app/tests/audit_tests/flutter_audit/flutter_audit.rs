//! Flutter audit report cases.
//! Các case report audit Flutter.

use crate::audit_tests::tmp;
use mgc_app_adapter::audit::scanner::{audit_flutter, audit_multi, dependency_health_flutter};
use mgc_types::adapter::ScannerStatus;

#[tokio::test]
async fn audit_flutter_security_is_unsupported_not_clean() {
    let dir = tmp("flutter-no-tool");
    std::fs::write(dir.join("pubspec.yaml"), "name: test\n").unwrap();

    // pub outdated is NOT a CVE scanner — the security audit must be
    // honestly UnsupportedEcosystem, never a fake clean report.
    // pub outdated KHÔNG phải scanner CVE — security audit phải trung thực
    // UnsupportedEcosystem, không bao giờ báo sạch giả.
    let report = audit_flutter(&dir).await.unwrap();
    assert_eq!(report.vulnerability_count, 0);
    assert!(matches!(
        report.scanner_status,
        mgc_types::adapter::ScannerStatus::UnsupportedEcosystem { .. }
    ));
}

#[tokio::test]
async fn dependency_health_flutter_without_flutter_fails_closed() {
    let dir = tmp("flutter-no-tool");
    std::fs::write(dir.join("pubspec.yaml"), "name: test\n").unwrap();

    // Without the CLI we cannot know freshness — an empty default report
    // would fake "everything up to date". Must fail closed.
    // Không có CLI thì không biết độ tươi — report rỗng đồng nghĩa bịa
    // "mọi thứ mới". Phải fail-closed.
    assert!(dependency_health_flutter(&dir).await.is_err());
}

#[tokio::test]
async fn audit_multi_detects_flutter_as_unsupported() {
    let dir = tmp("multi-flutter");
    std::fs::write(dir.join("pubspec.yaml"), "name: test\n").unwrap();

    let report = audit_multi(&dir).await.unwrap();
    assert_eq!(report.vulnerability_count, 0);
    assert!(matches!(
        report.scanner_status,
        ScannerStatus::UnsupportedEcosystem { .. }
    ));
}
