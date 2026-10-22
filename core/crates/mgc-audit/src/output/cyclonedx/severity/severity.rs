//! Project shared vulnerability levels to CycloneDX labels.
//! Ánh xạ level lỗ hổng dùng chung sang nhãn CycloneDX.

use mgc_types::adapter::VulnerabilitySeverity;

/// CycloneDX 1.5 severity label per contract level.
/// Nhãn severity CycloneDX 1.5 theo level của hợp đồng.
pub(in crate::output::cyclonedx) fn cyclonedx_severity(
    level: &VulnerabilitySeverity,
) -> &'static str {
    match level {
        VulnerabilitySeverity::Critical => "critical",
        VulnerabilitySeverity::High => "high",
        VulnerabilitySeverity::Medium => "medium",
        VulnerabilitySeverity::Low => "low",
        VulnerabilitySeverity::Info => "info",
    }
}
