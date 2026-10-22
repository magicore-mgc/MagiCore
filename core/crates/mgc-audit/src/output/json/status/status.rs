//! Map scanner execution states to stable JSON slugs.
//! Ánh xạ trạng thái chạy scanner sang slug JSON ổn định.

use mgc_types::adapter::{AuditReport, ScannerStatus};

pub(in crate::output::json) fn status_slug(report: &AuditReport) -> &'static str {
    match report.scanner_status {
        ScannerStatus::Available => "available",
        ScannerStatus::Partial { .. } => "partial",
        ScannerStatus::ToolMissing { .. } => "tool_missing",
        ScannerStatus::UnsupportedEcosystem { .. } => "unsupported_ecosystem",
        ScannerStatus::Failed { .. } => "failed",
    }
}
