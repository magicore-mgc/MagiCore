//! Flutter audit filesystem-boundary tests.
//! Test ranh giới filesystem của audit Flutter.

use super::*;
use mgc_types::adapter::ScannerStatus;

#[tokio::test]
async fn absent_lockfile_stays_explicitly_unsupported() {
    let directory = tempfile::tempdir().expect("create Flutter audit fixture directory");
    let report = run(directory.path())
        .await
        .expect("report absent Flutter lockfile");
    assert_eq!(report.vulnerability_count, 0);
    assert!(matches!(
        report.scanner_status,
        ScannerStatus::UnsupportedEcosystem { .. }
    ));
}
