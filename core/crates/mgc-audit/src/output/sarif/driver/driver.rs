//! Build SARIF driver metadata from scanner status.
//! Dựng metadata SARIF driver từ trạng thái scanner.

use super::super::types::SarifDriver;
use mgc_types::adapter::{AuditReport, ScannerStatus};

pub(in crate::output::sarif) fn build_driver(report: &AuditReport) -> SarifDriver {
    let verified = matches!(report.scanner_status, ScannerStatus::Available);
    let name = if verified {
        "mgc-audit".to_string()
    } else {
        "mgc-audit (UNVERIFIED — scanner did not complete)".to_string()
    };

    SarifDriver {
        name,
        version: env!("CARGO_PKG_VERSION").to_string(),
        information_uri: "https://github.com/magicore-mgc/MagiCore".to_string(),
    }
}
