//! Keep the Web-owned operational audit runner and stable public entry point.
//! Giữ runner audit Web và entry point public ổn định.

use mgc_types::adapter::AuditReport;
use mgc_types::{MgError, MgResult};
use std::path::Path;

use super::advisory_query::{AdvisoryQueryOutcome, query_npm_advisories};
use super::pin_source::select_audit_pins;
use super::report::{report_after_npm_scan, report_without_npm_pins};

/// The audited pins below come from the Web-owned lockfile seam.
/// Pin audit lấy từ seam lockfile do Web sở hữu.
pub async fn run_audit(project_root: &Path, registry_url: &str) -> MgResult<AuditReport> {
    let Some((npm_pins, jsr_pins)) = select_audit_pins(project_root).map_err(MgError::from)? else {
        return Ok(AuditReport::clean(0));
    };

    if npm_pins.is_empty() {
        return Ok(report_without_npm_pins(&jsr_pins));
    }

    let outcome = query_npm_advisories(registry_url, &npm_pins)
        .await
        .map_err(MgError::from)?;
    let vulnerabilities = match outcome {
        AdvisoryQueryOutcome::ScannerFailed(report) => return Ok(report),
        AdvisoryQueryOutcome::Parsed(vulnerabilities) => vulnerabilities,
    };

    Ok(report_after_npm_scan(&npm_pins, &jsr_pins, vulnerabilities))
}
