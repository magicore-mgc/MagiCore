//! Map Go advisory severity from database labels or CVSS.
//! (Ánh xạ severity advisory Go từ nhãn database hoặc CVSS.)

use super::super::model::GovulncheckOsv;
use mgc_types::adapter::VulnerabilitySeverity;

pub(in crate::scanners::govulncheck) fn govulncheck_severity(
    osv: &GovulncheckOsv,
) -> (String, VulnerabilitySeverity) {
    if let Some(label) = osv
        .database_specific
        .as_ref()
        .and_then(|d| d.severity.as_deref())
    {
        let level = match label.to_uppercase().as_str() {
            "CRITICAL" => VulnerabilitySeverity::Critical,
            "HIGH" => VulnerabilitySeverity::High,
            "MODERATE" | "MEDIUM" => VulnerabilitySeverity::Medium,
            "LOW" => VulnerabilitySeverity::Low,
            _ => VulnerabilitySeverity::Info,
        };
        return (label.to_uppercase().to_lowercase(), level);
    }
    if let Some(level) = osv
        .severity
        .iter()
        .find_map(|s| s.score.as_deref())
        .and_then(crate::scanners::cvss_base_severity_for_go)
    {
        let label = level.as_str().to_string();
        return (label, level);
    }
    ("info".to_string(), VulnerabilitySeverity::Info)
}
