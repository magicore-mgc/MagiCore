//! Versioned JSON audit envelope and pretty serializer.
//! Envelope audit JSON có version và serializer đẹp.

use super::super::skipped_reasons::skipped_reasons;
use super::super::status::status_slug;
use mgc_types::adapter::AuditReport;
use serde::Serialize;

/// Current envelope schema version — bump on breaking shape changes.
/// Version envelope hiện tại — tăng khi đổi shape gây vỡ.
pub const JSON_SCHEMA_VERSION: u32 = 1;

#[derive(Serialize)]
pub struct JsonAuditEnvelope<'a> {
    pub schema_version: u32,
    /// Scanner state: available | partial | tool_missing | unsupported_ecosystem | failed.
    /// Trạng thái scanner theo slug ổn định đã định nghĩa.
    pub scanner_status: &'a str,
    pub packages_audited: usize,
    pub vulnerability_count: usize,
    pub vulnerabilities: &'a [mgc_types::adapter::Vulnerability],
    /// Non-empty coverage reasons are additive to schema v1; empty values are omitted.
    /// Lý do coverage không rỗng được thêm vào schema v1; giá trị rỗng bị lược bỏ.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub skipped_reasons: Vec<String>,
}

impl<'a> JsonAuditEnvelope<'a> {
    pub fn from_report(report: &'a AuditReport) -> Self {
        Self {
            schema_version: JSON_SCHEMA_VERSION,
            scanner_status: status_slug(report),
            packages_audited: report.packages_audited,
            vulnerability_count: report.vulnerability_count,
            vulnerabilities: &report.vulnerabilities,
            skipped_reasons: skipped_reasons(report),
        }
    }

    /// Serialize the full envelope with stable pretty formatting.
    /// Serialize envelope đầy đủ với định dạng đẹp ổn định.
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
}
