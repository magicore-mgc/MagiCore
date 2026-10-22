//! Public aggregation coordinator — điều phối tổng hợp báo cáo audit.
use mgc_types::adapter::AuditReport;

use super::report_collection::collect_reports;
use super::status_resolution::resolve_status;

/// Aggregate every scanner report under the existing strictest-status policy.
/// Tổng hợp report của mọi scanner theo policy trạng thái nghiêm ngặt nhất hiện hành.
pub fn aggregate_reports(reports: Vec<(String, AuditReport)>) -> AuditReport {
    let mut summary = collect_reports(&reports);
    let scanner_status = resolve_status(&reports, &mut summary);

    AuditReport {
        packages_audited: summary.packages_audited,
        vulnerability_count: summary.vulnerabilities.len(),
        vulnerabilities: summary.vulnerabilities,
        scanner_status,
    }
}
