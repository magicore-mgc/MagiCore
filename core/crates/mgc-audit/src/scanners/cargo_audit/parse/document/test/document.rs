//! Tests for cargo-audit JSON document decoding.
//! Kiểm thử giải mã document JSON cargo-audit.

#![allow(clippy::unwrap_used)]

use super::super::error::CargoAuditParseError;
use super::*;

#[test]
fn leading_progress_and_unknown_metadata_remain_tolerated() {
    let report = deserialize_report(
        r#"progress: checking dependencies {"database":{"future":true},"vulnerabilities":{"found":false,"list":[]}}"#,
    )
    .unwrap();

    assert!(report.lockfile.is_none());
    assert!(!report.vulnerabilities.found);
    assert!(report.vulnerabilities.list.is_empty());
}

#[test]
fn missing_required_nested_fields_still_fail_closed() {
    let result =
        deserialize_report(r#"{"vulnerabilities":{"found":true,"list":[{"advisory":{}}]}}"#);

    assert!(matches!(result, Err(CargoAuditParseError::InvalidJson(_))));
}
