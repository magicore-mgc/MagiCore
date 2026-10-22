//! Dedupe coordinator tests — asserts that dry-run never reports mutations or GC.
//! Test coordinator dedupe — xác nhận dry-run không báo mutation hay GC.
use super::super::dry_run_report;

#[test]
fn dry_run_report_does_not_claim_saved_bytes_or_cleanup() {
    let report = dry_run_report(3, 2, 1);

    assert_eq!(report.before_instances, 3);
    assert_eq!(report.after_instances, 2);
    assert_eq!(report.merged, 1);
    assert_eq!(report.disk_saved_bytes, 0);
    assert!(report.entries.is_empty());
    assert!(report.gc_failed_paths.is_empty());
}
