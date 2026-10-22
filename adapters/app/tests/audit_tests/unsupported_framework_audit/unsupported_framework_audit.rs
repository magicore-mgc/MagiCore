//! Unsupported mobile-framework audit cases.
//! Các case audit framework mobile chưa hỗ trợ.

use crate::audit_tests::tmp;
use mgc_app_adapter::audit::scanner::{audit_cocoapods, audit_swift};
use mgc_types::adapter::ScannerStatus;

#[tokio::test]
async fn audit_maui_and_tauri_report_unsupported_ecosystem() {
    let dir = tmp("maui-tauri-unsupported");
    for (language, expected) in [
        (mgc_app_adapter::AppLanguage::Maui, "maui"),
        (mgc_app_adapter::AppLanguage::Tauri, "tauri"),
    ] {
        let report = mgc_app_adapter::audit::run_audit(language, &dir)
            .await
            .unwrap();
        assert_eq!(report.vulnerability_count, 0);
        assert!(matches!(
            report.scanner_status,
            ScannerStatus::UnsupportedEcosystem { ecosystem } if ecosystem == expected
        ));
    }
}

#[tokio::test]
async fn audit_swift_without_lockfile_is_unsupported_not_clean() {
    let dir = tmp("swift-audit");
    std::fs::write(dir.join("Package.swift"), "// swift package\n").unwrap();

    let report = audit_swift(&dir).await.unwrap();
    assert_eq!(report.vulnerability_count, 0);
    assert!(matches!(
        report.scanner_status,
        mgc_types::adapter::ScannerStatus::UnsupportedEcosystem { .. }
    ));
}

#[tokio::test]
async fn audit_cocoapods_without_lockfile_is_unsupported_not_clean() {
    let dir = tmp("cocoapods-audit");

    let report = audit_cocoapods(&dir).await.unwrap();
    assert_eq!(report.vulnerability_count, 0);
    assert!(matches!(
        report.scanner_status,
        mgc_types::adapter::ScannerStatus::UnsupportedEcosystem { .. }
    ));
}
