//! Verify unsupported and partial outcomes without network access.
//! Kiểm tra kết quả unsupported và partial không cần mạng.

use super::super::audit_dotnet;
use mgc_types::adapter::ScannerStatus;

#[tokio::test]
async fn no_project_is_reported_as_unsupported() {
    let project = tempfile::tempdir().unwrap();

    let report = audit_dotnet(project.path()).await.unwrap();

    match report.scanner_status {
        ScannerStatus::UnsupportedEcosystem { ecosystem } => {
            assert!(ecosystem.contains("no .sln/.csproj detected"));
        }
        other => panic!("expected UnsupportedEcosystem, got {other:?}"),
    }
    assert_eq!(report.vulnerability_count, 0);
}

#[tokio::test]
async fn solution_project_without_lock_is_reported_as_partial() {
    let project = tempfile::tempdir().unwrap();
    std::fs::write(project.path().join("Api.csproj"), "<Project/>\n").unwrap();
    std::fs::write(
        project.path().join("App.sln"),
        r#"Project("{A}") = "Api", "Api.csproj", "{B}""#,
    )
    .unwrap();

    let report = audit_dotnet(project.path()).await.unwrap();

    match report.scanner_status {
        ScannerStatus::Partial {
            scanned,
            skipped,
            reasons,
        } => {
            assert_eq!(scanned, 0);
            assert_eq!(skipped, 1);
            assert!(reasons[0].contains("no packages.lock.json"));
        }
        other => panic!("expected Partial, got {other:?}"),
    }
    assert_eq!(report.packages_audited, 0);
    assert_eq!(report.vulnerability_count, 0);
}
