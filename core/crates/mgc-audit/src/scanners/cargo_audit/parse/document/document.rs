//! Define and deserialize cargo-audit JSON document shapes.
//! Định nghĩa và deserialize shape document JSON cargo-audit.

use super::super::error::CargoAuditParseError;
use serde::Deserialize;

/// Required report fields stay strict while unknown metadata stays tolerated.
/// Field report bắt buộc vẫn chặt; metadata chưa biết vẫn được dung thứ.
#[derive(Debug, Deserialize)]
pub(in crate::scanners::cargo_audit::parse) struct CargoAuditReport {
    pub(in crate::scanners::cargo_audit::parse) lockfile: Option<CargoLockfile>,
    pub(in crate::scanners::cargo_audit::parse) vulnerabilities: CargoVulnerabilities,
}

#[derive(Debug, Deserialize)]
pub(in crate::scanners::cargo_audit::parse) struct CargoLockfile {
    #[serde(rename = "dependency-count")]
    pub(in crate::scanners::cargo_audit::parse) dependency_count: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub(in crate::scanners::cargo_audit::parse) struct CargoVulnerabilities {
    pub(in crate::scanners::cargo_audit::parse) found: bool,
    pub(in crate::scanners::cargo_audit::parse) count: Option<usize>,
    #[serde(default)]
    pub(in crate::scanners::cargo_audit::parse) list: Vec<CargoAuditFinding>,
}

/// One finding's advisory, optional patched versions and package identity.
/// Finding gồm advisory, patched version tùy chọn và danh tính package.
#[derive(Debug, Deserialize)]
pub(in crate::scanners::cargo_audit::parse) struct CargoAuditFinding {
    pub(in crate::scanners::cargo_audit::parse) advisory: CargoAdvisory,
    pub(in crate::scanners::cargo_audit::parse) versions: Option<CargoVersions>,
    pub(in crate::scanners::cargo_audit::parse) package: CargoPackage,
}

/// Advisory keys are forward-compatible; required id/title still fail closed.
/// Key advisory giữ tương thích tiến; thiếu id/title bắt buộc vẫn bị chặn.
#[derive(Debug, Deserialize)]
pub(in crate::scanners::cargo_audit::parse) struct CargoAdvisory {
    pub(in crate::scanners::cargo_audit::parse) id: String,
    pub(in crate::scanners::cargo_audit::parse) title: String,
    pub(in crate::scanners::cargo_audit::parse) cvss: Option<String>,
    #[serde(default)]
    pub(in crate::scanners::cargo_audit::parse) severity: Option<String>,
    pub(in crate::scanners::cargo_audit::parse) url: Option<String>,
    #[serde(default)]
    pub(in crate::scanners::cargo_audit::parse) aliases: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub(in crate::scanners::cargo_audit::parse) struct CargoVersions {
    #[serde(default)]
    pub(in crate::scanners::cargo_audit::parse) patched: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub(in crate::scanners::cargo_audit::parse) struct CargoPackage {
    pub(in crate::scanners::cargo_audit::parse) name: String,
    pub(in crate::scanners::cargo_audit::parse) version: String,
}

pub(in crate::scanners::cargo_audit::parse) fn deserialize_report(
    json: &str,
) -> Result<CargoAuditReport, CargoAuditParseError> {
    let json_start = json.find('{').unwrap_or(0);
    serde_json::from_str(json[json_start..].trim()).map_err(CargoAuditParseError::InvalidJson)
}
