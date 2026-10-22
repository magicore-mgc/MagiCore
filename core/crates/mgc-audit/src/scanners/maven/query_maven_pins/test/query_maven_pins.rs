//! Maven OSV report projection tests.
//! Test projection report OSV Maven.

use super::*;

#[test]
fn no_pins_and_no_skips_return_clean_zero_report_without_network() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build local runtime");
    let report = runtime
        .block_on(query_maven_pins(&(Vec::new(), Vec::new())))
        .expect("empty projection");
    assert_eq!(report.packages_audited, 0);
    assert!(report.is_clean());
}

#[test]
fn skipped_only_projection_returns_partial_without_network() {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build local runtime");
    let skipped = vec!["coordinate is incomplete".to_string()];
    let report = runtime
        .block_on(query_maven_pins(&(Vec::new(), skipped)))
        .expect("skip-only projection");
    assert!(matches!(
        report.scanner_status,
        ScannerStatus::Partial {
            scanned: 0,
            skipped: 1,
            ..
        }
    ));
}
