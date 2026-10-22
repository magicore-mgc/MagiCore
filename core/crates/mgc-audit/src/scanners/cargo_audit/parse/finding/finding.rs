//! Validate and convert a raw cargo-audit finding.
//! Xác thực và chuyển finding cargo-audit thô.

use super::super::document::CargoAuditFinding;
use super::super::error::CargoAuditParseError;
use super::super::severity::cargo_severity;
use mgc_types::adapter::{FindingClass, Vulnerability};

pub(in crate::scanners::cargo_audit::parse) fn parse_finding(
    finding: &CargoAuditFinding,
) -> Result<Vulnerability, CargoAuditParseError> {
    let pkg_name = mgc_types::PackageName::new(&finding.package.name).map_err(|_| {
        CargoAuditParseError::UnrepresentablePackageName {
            name: finding.package.name.clone(),
        }
    })?;
    let version = mgc_types::Version::parse(&finding.package.version).map_err(|_| {
        CargoAuditParseError::UnparseablePackageVersion {
            name: finding.package.name.clone(),
            version: finding.package.version.clone(),
        }
    })?;
    let (severity, severity_level) = cargo_severity(finding);

    Ok(Vulnerability {
        package: mgc_types::PackageId::new(pkg_name, version),
        title: format!("{}: {}", finding.advisory.id, finding.advisory.title),
        severity,
        cve: finding
            .advisory
            .aliases
            .iter()
            .find(|alias| alias.starts_with("CVE-"))
            .cloned()
            .unwrap_or_else(|| finding.advisory.id.clone()),
        severity_level,
        patched_versions: (!finding
            .versions
            .as_ref()
            .map(|versions| versions.patched.is_empty())
            .unwrap_or(true))
        .then(|| {
            finding
                .versions
                .as_ref()
                .map(|versions| versions.patched.join(", "))
                .unwrap_or_default()
        }),
        url: finding.advisory.url.clone(),
        scanner: None,
        ecosystem: None,
        evidence_at: None,
        finding_class: FindingClass::Vulnerability,
    }
    .with_evidence("cargo-audit", "rust"))
}
