//! Serde schema for the supported Go scanner stream fields.
//! (Schema serde cho các field stream scanner Go được hỗ trợ.)

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(in crate::scanners::govulncheck) struct GovulncheckMessage {
    #[serde(default, rename = "config")]
    pub(in crate::scanners::govulncheck) _config: Option<serde::de::IgnoredAny>,
    #[serde(default, rename = "progress")]
    pub(in crate::scanners::govulncheck) _progress: Option<serde::de::IgnoredAny>,
    /// SBOM message — module inventory (capital "SBOM" per the official
    /// JSON tags).
    /// Thông điệp SBOM — danh sách module (viết hoa "SBOM" theo JSON tag
    /// chính thức).
    #[serde(default, rename = "SBOM")]
    pub(in crate::scanners::govulncheck) sbom: Option<GovulncheckSbom>,
    /// OSV entry — advisory metadata keyed by `id`.
    /// Entry OSV — metadata advisory, khóa theo `id`.
    #[serde(default, rename = "osv")]
    pub(in crate::scanners::govulncheck) osv: Option<GovulncheckOsv>,
    #[serde(default, rename = "finding")]
    pub(in crate::scanners::govulncheck) finding: Option<GovulncheckFinding>,
}

/// SBOM message: the exact module set scanned — feeds packages_audited.
/// Thông điệp SBOM: đúng tập module đã quét — dùng cho packages_audited.
#[derive(Debug, Deserialize)]
pub(in crate::scanners::govulncheck) struct GovulncheckSbom {
    #[serde(default)]
    pub(in crate::scanners::govulncheck) modules: Vec<GovulncheckModule>,
}

/// One SBOM module — only the count matters for packages_audited; the
/// per-module path/version stay in the payload for schema completeness.
/// Một module SBOM — chỉ SỐ LƯỢNG dùng cho packages_audited; path/
/// version giữ trong payload cho đủ schema.
#[derive(Debug, Deserialize)]
pub(in crate::scanners::govulncheck) struct GovulncheckModule {
    #[serde(default)]
    #[allow(dead_code)]
    pub(in crate::scanners::govulncheck) path: String,
    #[serde(default)]
    #[allow(dead_code)]
    pub(in crate::scanners::govulncheck) version: String,
}

/// OSV advisory entry — the subset of the OSV schema govulncheck emits.
/// Entry advisory OSV — tập con schema OSV mà govulncheck phát hành.
#[derive(Debug, Deserialize)]
pub(in crate::scanners::govulncheck) struct GovulncheckOsv {
    pub(in crate::scanners::govulncheck) id: String,
    #[serde(default)]
    pub(in crate::scanners::govulncheck) summary: String,
    #[serde(default)]
    pub(in crate::scanners::govulncheck) details: String,
    #[serde(default)]
    pub(in crate::scanners::govulncheck) aliases: Vec<String>,
    #[serde(default)]
    pub(in crate::scanners::govulncheck) severity: Vec<GovulncheckSeverity>,
    #[serde(default)]
    pub(in crate::scanners::govulncheck) database_specific: Option<GovulncheckDbSpecific>,
    #[serde(default)]
    pub(in crate::scanners::govulncheck) references: Vec<GovulncheckReference>,
}

#[derive(Debug, Deserialize)]
pub(in crate::scanners::govulncheck) struct GovulncheckSeverity {
    /// CVSS vector string ("CVSS:3.1/AV:...").
    /// Chuỗi vector CVSS ("CVSS:3.1/AV:...").
    pub(in crate::scanners::govulncheck) score: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(in crate::scanners::govulncheck) struct GovulncheckDbSpecific {
    /// Go vulndb severity label: HIGH | MODERATE | LOW | (TEMPORARY...).
    /// Nhãn severity của Go vulndb: HIGH | MODERATE | LOW | (TEMPORARY...).
    #[serde(default)]
    pub(in crate::scanners::govulncheck) severity: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(in crate::scanners::govulncheck) struct GovulncheckReference {
    #[serde(default)]
    pub(in crate::scanners::govulncheck) url: Option<String>,
}

/// One finding: `osv` id + trace; the FIRST trace frame carries the
/// affected module/version.
/// Một finding: id `osv` + trace; frame ĐẦU của trace chứa module/version
/// bị dính.
#[derive(Debug, Deserialize)]
pub(in crate::scanners::govulncheck) struct GovulncheckFinding {
    #[serde(default)]
    pub(in crate::scanners::govulncheck) osv: String,
    #[serde(default)]
    pub(in crate::scanners::govulncheck) fixed_version: Option<String>,
    #[serde(default)]
    pub(in crate::scanners::govulncheck) trace: Vec<GovulncheckFrame>,
}

#[derive(Debug, Deserialize)]
pub(in crate::scanners::govulncheck) struct GovulncheckFrame {
    #[serde(default)]
    pub(in crate::scanners::govulncheck) module: String,
    #[serde(default)]
    pub(in crate::scanners::govulncheck) version: String,
}
