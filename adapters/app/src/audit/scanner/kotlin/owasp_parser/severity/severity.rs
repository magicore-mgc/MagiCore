//! Maps the existing CVSS score bands and fallback severity string.
//! Ánh xạ các ngưỡng CVSS và chuỗi severity dự phòng hiện có.

use super::super::schema::OwaspVulnerability;
use mgc_types::adapter::VulnerabilitySeverity;
/// Map dependency-check severity: CVSS v3/v2 base score wins; else the
/// severity string. Bands follow CVSS qualitative ratings.
/// Map severity dependency-check: ưu tiên base score CVSS v3/v2; còn lại
/// theo chuỗi severity. Bảng theo xếp loại định tính CVSS.
#[cfg(test)]
pub(in crate::audit::scanner::kotlin::owasp_parser) fn owasp_severity(
    vuln: &OwaspVulnerability,
) -> VulnerabilitySeverity {
    let score = vuln
        .cvssv3
        .as_ref()
        .and_then(|c| c.base_score)
        .or_else(|| vuln.cvssv2.as_ref().and_then(|c| c.base_score));
    match score {
        Some(s) if s >= 9.0 => VulnerabilitySeverity::Critical,
        Some(s) if s >= 7.0 => VulnerabilitySeverity::High,
        Some(s) if s >= 4.0 => VulnerabilitySeverity::Medium,
        Some(s) if s > 0.0 => VulnerabilitySeverity::Low,
        Some(_) => VulnerabilitySeverity::Info,
        None => VulnerabilitySeverity::from_str(vuln.severity.as_deref().unwrap_or("")),
    }
}
