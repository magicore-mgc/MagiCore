//! Select cargo-audit label, shared CVSS severity or Info fallback.
//! Chọn nhãn cargo-audit, severity CVSS dùng chung hoặc fallback Info.

use super::super::super::cvss_base_severity;
use super::super::document::CargoAuditFinding;
use mgc_types::adapter::VulnerabilitySeverity;

pub(in crate::scanners::cargo_audit::parse) fn cargo_severity(
    finding: &CargoAuditFinding,
) -> (String, VulnerabilitySeverity) {
    if let Some(label) = &finding.advisory.severity
        && !label.is_empty()
    {
        let level = VulnerabilitySeverity::from_str(label);
        return (label.to_lowercase(), level);
    }
    if let Some(level) = finding
        .advisory
        .cvss
        .as_deref()
        .and_then(cvss_base_severity)
    {
        let label = level.as_str().to_string();
        return (label.clone(), level);
    }
    ("info".to_string(), VulnerabilitySeverity::Info)
}
