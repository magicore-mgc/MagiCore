//! Tests for JSON envelope shape, schema and report projection.
//! Kiểm thử shape envelope JSON, schema và ánh xạ report.

#![allow(clippy::unwrap_used)]

use super::*;
use mgc_types::adapter::{AuditReport, ScannerStatus};

#[test]
fn available_envelope_keeps_v1_schema_and_omits_empty_reasons() {
    let report = AuditReport::clean(3);
    let envelope = JsonAuditEnvelope::from_report(&report);

    assert_eq!(envelope.schema_version, JSON_SCHEMA_VERSION);
    assert_eq!(envelope.scanner_status, "available");
    assert!(std::ptr::eq(
        envelope.vulnerabilities,
        report.vulnerabilities.as_slice()
    ));

    let serialized = serde_json::to_value(&envelope).unwrap();
    assert_eq!(
        serialized["schema_version"].as_u64(),
        Some(u64::from(JSON_SCHEMA_VERSION))
    );
    assert_eq!(serialized["scanner_status"], "available");
    assert_eq!(serialized["packages_audited"], 3);
    assert_eq!(serialized["vulnerability_count"], 0);
    assert!(serialized.get("skipped_reasons").is_none());

    let pretty = envelope.to_json().unwrap();
    assert!(pretty.contains('\n'));
}

#[test]
fn partial_envelope_keeps_ordered_reasons_and_serializes_as_json() {
    let report = AuditReport {
        packages_audited: 2,
        vulnerability_count: 0,
        vulnerabilities: Vec::new(),
        scanner_status: ScannerStatus::Partial {
            scanned: 1,
            skipped: 1,
            reasons: vec![
                "lockfile missing".to_string(),
                "transitive graph unknown".to_string(),
            ],
        },
    };
    let envelope = JsonAuditEnvelope::from_report(&report);

    let serialized = serde_json::to_value(&envelope).unwrap();
    assert_eq!(serialized["scanner_status"], "partial");
    assert_eq!(
        serialized["skipped_reasons"],
        serde_json::json!(["lockfile missing", "transitive graph unknown"])
    );
    let pretty_value =
        serde_json::from_str::<serde_json::Value>(&envelope.to_json().unwrap()).unwrap();
    assert_eq!(pretty_value, serialized);
}
