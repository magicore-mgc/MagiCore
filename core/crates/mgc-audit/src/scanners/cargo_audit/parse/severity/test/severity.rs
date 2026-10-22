//! Tests for cargo-audit severity precedence.
//! Kiểm thử thứ tự ưu tiên severity cargo-audit.

use super::super::document::{CargoAdvisory, CargoAuditFinding, CargoPackage};
use super::*;
use mgc_types::adapter::VulnerabilitySeverity;

fn finding(severity: Option<&str>, cvss: Option<&str>) -> CargoAuditFinding {
    CargoAuditFinding {
        advisory: CargoAdvisory {
            id: "RUSTSEC-2026-0002".to_string(),
            title: "Severity test".to_string(),
            cvss: cvss.map(str::to_string),
            severity: severity.map(str::to_string),
            url: None,
            aliases: Vec::new(),
        },
        versions: None,
        package: CargoPackage {
            name: "serde".to_string(),
            version: "1.2.0".to_string(),
        },
    }
}

#[test]
fn advisory_label_wins_over_cvss_vector_and_is_lowercased() {
    let (label, severity) = cargo_severity(&finding(
        Some("HIGH"),
        Some("CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:H"),
    ));

    assert_eq!(label, "high");
    assert_eq!(severity, VulnerabilitySeverity::High);
}

#[test]
fn absent_label_uses_the_existing_shared_cvss_calculator() {
    let (label, severity) = cargo_severity(&finding(
        None,
        Some("CVSS:3.1/AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:H"),
    ));

    assert_eq!(label, "critical");
    assert_eq!(severity, VulnerabilitySeverity::Critical);
}

#[test]
fn missing_or_invalid_severity_data_defaults_to_info() {
    assert_eq!(
        cargo_severity(&finding(None, None)),
        ("info".to_string(), VulnerabilitySeverity::Info)
    );
    assert_eq!(
        cargo_severity(&finding(None, Some("not-a-cvss-vector"))),
        ("info".to_string(), VulnerabilitySeverity::Info)
    );
}
