//! Collect report rows and scanner-state facts — gom report và trạng thái scanner.
use mgc_types::adapter::{AuditReport, ScannerStatus, Vulnerability};

/// Internal values collected once and shared with the status decision owner.
/// Dữ liệu nội bộ được gom một lượt rồi dùng chung cho owner phân xử trạng thái.
pub(in crate::aggregate) struct ReportSummary {
    pub(in crate::aggregate) packages_audited: usize,
    pub(in crate::aggregate) vulnerabilities: Vec<Vulnerability>,
    pub(in crate::aggregate) scanned: usize,
    pub(in crate::aggregate) skipped: usize,
    pub(in crate::aggregate) reasons: Vec<String>,
    pub(in crate::aggregate) any_failed: Option<(String, String)>,
    pub(in crate::aggregate) any_tool_missing: Option<(String, String)>,
    pub(in crate::aggregate) all_unsupported: bool,
    pub(in crate::aggregate) any_available: bool,
}

/// Fold all scanner rows without discarding findings from partial scans.
/// Gom mọi row scanner và không bỏ finding từ lượt scan chưa đầy đủ.
pub(in crate::aggregate) fn collect_reports(reports: &[(String, AuditReport)]) -> ReportSummary {
    let mut packages_audited = 0usize;
    let mut vulnerabilities = Vec::new();
    let mut scanned: usize = 0;
    let mut skipped: usize = 0;
    let mut reasons: Vec<String> = Vec::new();

    // Track the dominant state across steps.
    // Theo dõi trạng thái chi phối qua các bước.
    let mut any_failed: Option<(String, String)> = None;
    let mut any_tool_missing: Option<(String, String)> = None;
    let mut all_unsupported = true;
    let mut any_available = false;

    for (label, report) in reports {
        match &report.scanner_status {
            ScannerStatus::Available => {
                all_unsupported = false;
                any_available = true;
                packages_audited += report.packages_audited;
                vulnerabilities.extend(report.vulnerabilities.iter().cloned());
                scanned += 1;
            }
            ScannerStatus::Partial {
                scanned: scanned_n,
                skipped: skipped_n,
                reasons: r,
            } => {
                all_unsupported = false;
                // A Partial step keeps real findings and scanned packages.
                // Step Partial giữ finding thật và package đã scan.
                packages_audited += report.packages_audited;
                vulnerabilities.extend(report.vulnerabilities.iter().cloned());
                scanned += scanned_n;
                skipped += skipped_n;
                reasons.extend(r.iter().cloned());
            }
            ScannerStatus::ToolMissing { tool, remediation } => {
                all_unsupported = false;
                if any_tool_missing.is_none() {
                    any_tool_missing = Some((format!("{label}: {tool}"), remediation.clone()));
                }
            }
            ScannerStatus::UnsupportedEcosystem { ecosystem } => {
                reasons.push(format!("{label}: no scanner implemented for '{ecosystem}'"));
            }
            ScannerStatus::Failed { scanner, reason } => {
                all_unsupported = false;
                if any_failed.is_none() {
                    any_failed = Some((scanner.clone(), reason.clone()));
                }
            }
        }
    }

    ReportSummary {
        packages_audited,
        vulnerabilities,
        scanned,
        skipped,
        reasons,
        any_failed,
        any_tool_missing,
        all_unsupported,
        any_available,
    }
}

#[cfg(test)]
#[path = "test/report_collection.rs"]
mod tests;
