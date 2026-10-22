//! Resolve the strictest scanner status — phân xử trạng thái nghiêm ngặt nhất.
use mgc_types::adapter::{AuditReport, ScannerStatus};

use crate::aggregate::report_collection::ReportSummary;

/// Preserve the established priority and partial-coverage behavior.
/// Giữ nguyên thứ tự ưu tiên và cách biểu diễn độ phủ Partial.
pub(in crate::aggregate) fn resolve_status(
    reports: &[(String, AuditReport)],
    summary: &mut ReportSummary,
) -> ScannerStatus {
    if let Some((scanner, reason)) = summary.any_failed.take() {
        ScannerStatus::Failed { scanner, reason }
    } else if let Some((tool, remediation)) = summary.any_tool_missing.take() {
        ScannerStatus::ToolMissing { tool, remediation }
    } else if summary.all_unsupported && !reports.is_empty() {
        // Nothing ran at all — no scanner for ANY recognized manifest.
        // Không scanner nào chạy — không có scanner cho bất kỳ manifest nào.
        let ecosystem = reports
            .first()
            .and_then(|(_, report)| match &report.scanner_status {
                ScannerStatus::UnsupportedEcosystem { ecosystem } => Some(ecosystem.clone()),
                _ => None,
            })
            .unwrap_or_else(|| "multi".to_string());
        ScannerStatus::UnsupportedEcosystem { ecosystem }
    } else if summary.any_available && summary.reasons.is_empty() {
        ScannerStatus::Available
    } else if summary.any_available {
        ScannerStatus::Partial {
            scanned: summary.scanned,
            skipped: summary.skipped,
            reasons: std::mem::take(&mut summary.reasons),
        }
    } else if reports.is_empty() {
        // No manifests detected at all — nothing to audit, honest empty.
        // Không nhận diện manifest nào — không có gì để audit, đầu ra rỗng trung thực.
        ScannerStatus::Available
    } else {
        ScannerStatus::Partial {
            scanned: summary.scanned,
            skipped: summary.skipped,
            reasons: std::mem::take(&mut summary.reasons),
        }
    }
}

#[cfg(test)]
#[path = "test/status_resolution.rs"]
mod tests;
