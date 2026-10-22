//! Tests for cargo-audit finding validation and projection.
//! Kiểm thử validation và ánh xạ finding cargo-audit.

#![allow(clippy::unwrap_used)]

use super::super::document::{CargoAdvisory, CargoAuditFinding, CargoPackage, CargoVersions};
use super::super::error::CargoAuditParseError;
use super::*;

fn finding(name: &str, version: &str) -> CargoAuditFinding {
    CargoAuditFinding {
        advisory: CargoAdvisory {
            id: "RUSTSEC-2026-0001".to_string(),
            title: "Example advisory".to_string(),
            cvss: None,
            severity: Some("high".to_string()),
            url: Some("https://example.invalid/advisory".to_string()),
            aliases: vec!["GHSA-example".to_string(), "CVE-2026-0001".to_string()],
        },
        versions: Some(CargoVersions {
            patched: vec![">=1.2.3".to_string()],
        }),
        package: CargoPackage {
            name: name.to_string(),
            version: version.to_string(),
        },
    }
}

#[test]
fn valid_finding_preserves_alias_patch_and_evidence() {
    let vulnerability = parse_finding(&finding("serde", "1.2.0")).unwrap();

    assert_eq!(vulnerability.package.name().as_str(), "serde");
    assert_eq!(vulnerability.package.version().to_string(), "1.2.0");
    assert_eq!(vulnerability.title, "RUSTSEC-2026-0001: Example advisory");
    assert_eq!(vulnerability.cve, "CVE-2026-0001");
    assert_eq!(vulnerability.patched_versions.as_deref(), Some(">=1.2.3"));
    assert_eq!(vulnerability.scanner.as_deref(), Some("cargo-audit"));
    assert_eq!(vulnerability.ecosystem.as_deref(), Some("rust"));
    assert!(vulnerability.evidence_at.is_some());
}

#[test]
fn missing_cve_alias_falls_back_to_advisory_id_and_empty_patch_is_none() {
    let mut raw = finding("serde", "1.2.0");
    raw.advisory.aliases.clear();
    raw.versions.as_mut().unwrap().patched.clear();

    let vulnerability = parse_finding(&raw).unwrap();

    assert_eq!(vulnerability.cve, "RUSTSEC-2026-0001");
    assert!(vulnerability.patched_versions.is_none());
}

#[test]
fn invalid_package_name_and_version_return_typed_errors() {
    assert!(matches!(
        parse_finding(&finding("bad name", "1.2.0")),
        Err(CargoAuditParseError::UnrepresentablePackageName { .. })
    ));
    assert!(matches!(
        parse_finding(&finding("serde", "not-a-version")),
        Err(CargoAuditParseError::UnparseablePackageVersion { .. })
    ));
}
