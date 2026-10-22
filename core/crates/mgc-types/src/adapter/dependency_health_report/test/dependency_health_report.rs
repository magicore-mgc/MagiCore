//! Dependency Health Report owner contract test.
//! Kiểm tra hợp đồng báo cáo độ mới dependency.

use crate::adapter::DependencyHealthReport;

#[test]
fn default_report_has_zero_counts_and_no_outdated_entries() {
    let report = DependencyHealthReport::default();
    assert_eq!(report.packages_checked, 0);
    assert_eq!(report.outdated_count, 0);
    assert!(report.outdated.is_empty());
}
