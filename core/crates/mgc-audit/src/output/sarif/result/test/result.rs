//! Tests for SARIF vulnerability result projection.
//! Kiểm thử ánh xạ vulnerability thành SARIF result.

#![allow(clippy::unwrap_used)]

use super::*;
use mgc_types::adapter::{FindingClass, Vulnerability, VulnerabilitySeverity};
use mgc_types::{PackageId, PackageName, Version};

fn vulnerability(severity_level: VulnerabilitySeverity) -> Vulnerability {
    Vulnerability {
        package: PackageId::new(
            PackageName::new("serde".to_string()).unwrap(),
            Version::parse("1.0.0").unwrap(),
        ),
        title: "Security advisory".to_string(),
        severity: severity_level.as_str().to_string(),
        cve: "GHSA-example".to_string(),
        severity_level,
        patched_versions: None,
        url: None,
        scanner: Some("mgc-osv".to_string()),
        ecosystem: Some("crates.io".to_string()),
        evidence_at: Some("2026-01-02T03:04:05Z".to_string()),
        finding_class: FindingClass::Vulnerability,
    }
}

#[test]
fn severity_groups_and_result_projection_keep_existing_values() {
    let cases = [
        (VulnerabilitySeverity::Critical, "error"),
        (VulnerabilitySeverity::High, "error"),
        (VulnerabilitySeverity::Medium, "warning"),
        (VulnerabilitySeverity::Low, "warning"),
        (VulnerabilitySeverity::Info, "note"),
    ];
    for (severity, expected_level) in cases {
        let result = map_vulnerability(&vulnerability(severity));
        assert_eq!(result.level, expected_level);
        assert_eq!(result.rule_id, "GHSA-example");
        assert_eq!(
            result.message.text,
            "Security advisory [vulnerability] in serde@1.0.0 (scanner: mgc-osv, evidence: 2026-01-02T03:04:05Z)"
        );
        assert_eq!(
            result.locations[0].logical_locations[0].name,
            "crates.io:serde@1.0.0"
        );
    }
}

#[test]
fn missing_provenance_fields_keep_existing_fallback_text() {
    let mut finding = vulnerability(VulnerabilitySeverity::Medium);
    finding.scanner = None;
    finding.ecosystem = None;
    finding.evidence_at = None;

    let result = map_vulnerability(&finding);

    assert_eq!(
        result.message.text,
        "Security advisory [vulnerability] in serde@1.0.0 (scanner: unknown, evidence: not-stamped)"
    );
    assert_eq!(
        result.locations[0].logical_locations[0].name,
        "unknown:serde@1.0.0"
    );
}
