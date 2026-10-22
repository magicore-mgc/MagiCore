//! Preserve scanner-failed classification for non-success registry responses.
//! Giữ phân loại scanner-failed khi registry trả HTTP không thành công.

use mgc_types::adapter::ScannerStatus;

#[tokio::test]
async fn query_maps_http_failure_to_scanner_failed() {
    let (registry, server) = super::support::start_registry("503 Service Unavailable", "{}").await;
    let result =
        super::query_npm_advisories(&registry, &[("leftpad".to_string(), "1.0.0".to_string())])
            .await
            .unwrap();
    let _request = server.await.unwrap();
    let super::AdvisoryQueryOutcome::ScannerFailed(report) = result else {
        panic!("non-success registry response must become scanner-failed");
    };
    assert!(matches!(
        report.scanner_status,
        ScannerStatus::Failed { ref scanner, ref reason }
            if scanner == "npm-bulk-advisory" && reason.contains("503 Service Unavailable")
    ));
}
