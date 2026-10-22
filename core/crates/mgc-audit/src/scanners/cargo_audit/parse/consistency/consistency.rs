//! Validate cargo-audit's count and found/list consistency fields.
//! Xác thực count và found/list trong report cargo-audit.

use super::super::document::CargoVulnerabilities;
use super::super::error::CargoAuditParseError;

pub(in crate::scanners::cargo_audit::parse) fn validate_vulnerabilities(
    vulnerabilities: &CargoVulnerabilities,
) -> Result<(), CargoAuditParseError> {
    if let Some(count) = vulnerabilities.count
        && count != vulnerabilities.list.len()
    {
        return Err(CargoAuditParseError::CountMismatch {
            count,
            actual: vulnerabilities.list.len(),
        });
    }
    if vulnerabilities.found != !vulnerabilities.list.is_empty() {
        return Err(CargoAuditParseError::FoundListMismatch);
    }
    Ok(())
}
