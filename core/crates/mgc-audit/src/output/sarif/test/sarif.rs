//! Tests for SARIF run assembly and serialized schema.
//! Kiểm thử lắp ráp SARIF run và schema serialize.

#![allow(clippy::unwrap_used)]

use super::*;
use mgc_types::adapter::{
    AuditReport, FindingClass, ScannerStatus, Vulnerability, VulnerabilitySeverity,
};
use mgc_types::{PackageId, PackageName, Version};

fn vulnerability() -> Vulnerability {
    Vulnerability {
        package: PackageId::new(
            PackageName::new("serde".to_string()).unwrap(),
            Version::parse("1.0.0").unwrap(),
        ),
        title: "Security advisory".to_string(),
        severity: "high".to_string(),
        cve: "GHSA-example".to_string(),
        severity_level: VulnerabilitySeverity::High,
        patched_versions: None,
        url: None,
        scanner: Some("mgc-osv".to_string()),
        ecosystem: Some("crates.io".to_string()),
        evidence_at: Some("2026-01-02T03:04:05Z".to_string()),
        finding_class: FindingClass::Vulnerability,
    }
}

#[test]
fn non_available_report_preserves_supplied_findings_and_serializes_schema() {
    let report = AuditReport {
        packages_audited: 1,
        vulnerability_count: 1,
        vulnerabilities: vec![vulnerability()],
        scanner_status: ScannerStatus::Failed {
            scanner: "mgc-osv".to_string(),
            reason: "request failed".to_string(),
        },
    };

    let log = to_sarif(&report);
    assert_eq!(log.runs.len(), 1);
    assert_eq!(
        log.runs[0].tool.driver.name,
        "mgc-audit (UNVERIFIED — scanner did not complete)"
    );
    assert_eq!(log.runs[0].results.len(), 1);
    assert_eq!(log.runs[0].results[0].rule_id, "GHSA-example");

    let serialized = serde_json::to_value(&log).unwrap();
    assert_eq!(serialized["$schema"], SARIF_SCHEMA);
    assert_eq!(serialized["version"], SARIF_VERSION);
    assert_eq!(serialized["runs"][0]["results"][0]["level"], "error");
}

/// An EMPTY report must NOT use the trusted driver name.
///
/// `AuditReport::clean(0)` means nothing was inspected, so the SARIF consumer has
/// to see the unverified driver — otherwise a CI dashboard ingesting this file
/// records a passing scan for an unexamined project.
/// (`clean(0)` nghĩa là không có gì được kiểm, nên SARIF phải mang driver
/// UNVERIFIED — nếu không dashboard CI ghi nhận một lần scan đạt cho project chưa
/// được xem.)
#[test]
fn an_empty_report_uses_the_unverified_driver_name() {
    let log = to_sarif(&AuditReport::clean(0));

    assert_eq!(log.runs.len(), 1);
    assert_eq!(
        log.runs[0].tool.driver.name,
        "mgc-audit (UNVERIFIED — scanner did not complete)"
    );
    assert!(log.runs[0].results.is_empty());
}

/// A report with at least one inspected package keeps the trusted name.
/// (Report có ít nhất một package đã kiểm giữ tên tin cậy.)
#[test]
fn a_non_empty_report_keeps_the_trusted_driver_name() {
    let log = to_sarif(&AuditReport::clean(1));
    assert_eq!(log.runs[0].tool.driver.name, "mgc-audit");
}
