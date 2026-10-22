//! Parse the existing OWASP dependency-check JSON fixture schema.
//! Parse schema JSON OWASP dependency-check dùng trong regression fixture.

use super::{
    purl_name_version::purl_name_version, schema::OwaspReport, severity::owasp_severity,
    truncate_utf8::truncate_utf8,
};
use mgc_types::adapter::{AuditReport, FindingClass, ScannerStatus, Vulnerability};
use mgc_types::{MgError, MgResult, PackageId, PackageName, Version};

/// Parse an OWASP dependency-check JSON report into an audit report.
/// Parse report JSON của dependency-check thành audit report.
#[cfg(test)]
pub(crate) fn parse_owasp_dependency_check_json(raw: &str) -> MgResult<AuditReport> {
    let report: OwaspReport = serde_json::from_str(raw)
        .map_err(|e| MgError::Other(format!("invalid dependency-check JSON: {e}")))?;

    let mut vulnerabilities = Vec::new();
    for dep in &report.dependencies {
        // A dep without a purl cannot be attributed — count it audited but
        // raise on findings we cannot place; never silently drop them.
        // Dep không có purl thì không gán finding được — đếm là đã audit
        // nhưng finding không đặt được thì phải nêu, không bỏ âm thầm.
        let (pkg_name, pkg_version) = dep
            .packages
            .first()
            .map(|w| purl_name_version(&w.package.id))
            .unwrap_or_else(|| (PackageName::new("artifact").expect("static name"), None));
        for vuln in &dep.vulnerabilities {
            let severity_level = owasp_severity(vuln);
            let description = vuln.description.as_deref().unwrap_or("");
            vulnerabilities.push(
                Vulnerability {
                    package: PackageId::new(
                        pkg_name.clone(),
                        pkg_version.clone().unwrap_or_else(|| Version::new(0, 0, 0)),
                    ),
                    title: format!("{}: {}", vuln.name, truncate_utf8(description, 200)),
                    severity: vuln
                        .severity
                        .clone()
                        .unwrap_or_else(|| "unknown".to_string()),
                    cve: vuln.name.clone(),
                    severity_level,
                    patched_versions: None,
                    url: None,
                    scanner: None,
                    ecosystem: None,
                    evidence_at: None,
                    finding_class: FindingClass::Vulnerability,
                }
                .with_evidence("owasp-dependency-check", "kotlin/jvm"),
            );
        }
    }

    let count = vulnerabilities.len();
    Ok(AuditReport {
        packages_audited: report.dependencies.len(),
        vulnerability_count: count,
        vulnerabilities,
        scanner_status: ScannerStatus::Available,
    })
}
