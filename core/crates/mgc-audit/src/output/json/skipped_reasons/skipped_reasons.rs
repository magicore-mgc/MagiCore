//! Map scanner states to ordered JSON coverage reasons.
//! Ánh xạ trạng thái scanner sang lý do coverage theo thứ tự JSON.

use mgc_types::adapter::{AuditReport, ScannerStatus};

pub(in crate::output::json) fn skipped_reasons(report: &AuditReport) -> Vec<String> {
    match &report.scanner_status {
        ScannerStatus::Partial { reasons, .. } => reasons.clone(),
        ScannerStatus::ToolMissing { tool, remediation } => {
            vec![format!("{tool}: {remediation}")]
        }
        ScannerStatus::UnsupportedEcosystem { ecosystem } => vec![ecosystem.clone()],
        ScannerStatus::Failed { scanner, reason } => vec![format!("{scanner}: {reason}")],
        ScannerStatus::Available => Vec::new(),
    }
}
