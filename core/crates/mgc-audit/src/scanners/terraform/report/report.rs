//! Project Terraform parser state into the shared audit report.
//! Ánh xạ parser state Terraform sang audit report dùng chung.

use super::super::types::TerraformLockState;
use mgc_types::adapter::{AuditReport, ScannerStatus};

pub(crate) fn project(state: TerraformLockState) -> AuditReport {
    if !state.failures.is_empty() {
        return AuditReport::scanner_failed("terraform-provider-lock", state.failures.join("; "));
    }
    AuditReport {
        packages_audited: state.providers,
        vulnerability_count: 0,
        vulnerabilities: vec![],
        scanner_status: ScannerStatus::Available,
    }
}

#[cfg(test)]
#[path = "test/report.rs"]
mod tests;
