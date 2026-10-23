//! Map an MGC vulnerability into its SARIF result row.
//! Ánh xạ vulnerability của MGC thành dòng SARIF result.

use super::super::types::{SarifLocation, SarifLogicalLocation, SarifMessage, SarifResult};
use mgc_types::adapter::{Vulnerability, VulnerabilitySeverity};

pub(in crate::output::sarif) fn map_vulnerability(vulnerability: &Vulnerability) -> SarifResult {
    SarifResult {
        level: match vulnerability.severity_level {
            VulnerabilitySeverity::Critical | VulnerabilitySeverity::High => "error",
            VulnerabilitySeverity::Medium | VulnerabilitySeverity::Low => "warning",
            VulnerabilitySeverity::Info => "note",
        },
        message: SarifMessage {
            text: format!(
                "{} [{}] in {} (scanner: {}, evidence: {})",
                vulnerability.title,
                vulnerability.finding_class,
                vulnerability.package,
                vulnerability.scanner.as_deref().unwrap_or("unknown"),
                vulnerability
                    .evidence_at
                    .as_deref()
                    .unwrap_or("not-stamped"),
            ),
        },
        rule_id: vulnerability.cve.clone(),
        locations: vec![SarifLocation {
            logical_locations: vec![SarifLogicalLocation {
                name: format!(
                    "{}:{}@{}",
                    vulnerability.ecosystem.as_deref().unwrap_or("unknown"),
                    vulnerability.package.name().as_str(),
                    vulnerability.package.version(),
                ),
            }],
        }],
    }
}
