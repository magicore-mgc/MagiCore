//! severity precedence and CVSS fallback.
//! Ánh xạ mức severity theo thứ tự ưu tiên đã quy định.

use super::super::OsvVuln;
use mgc_types::adapter::VulnerabilitySeverity;

/// Severity: database_specific label > CVSS vector (shared base
/// implementation) > honest Info.
/// Severity: nhãn database_specific > vector CVSS (dùng chung bản cơ
/// sở) > Info trung thực.
pub(crate) fn osv_severity(vuln: &OsvVuln) -> (String, VulnerabilitySeverity) {
    if let Some(label) = vuln
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
        return (label.to_lowercase(), level);
    }
    if let Some(level) = vuln
        .severity
        .iter()
        .find_map(|s| s.score.as_deref())
        .and_then(crate::scanners::cvss_base_severity_for_go)
    {
        return (level.as_str().to_string(), level);
    }
    ("info".to_string(), VulnerabilitySeverity::Info)
}
