//! Tests for typed cargo-audit parse errors and display compatibility.
//! Kiểm thử lỗi parse cargo-audit typed và tương thích message.

#![allow(clippy::unwrap_used)]

use super::*;
use mgc_types::MgError;
use std::error::Error;

#[test]
fn serde_failure_keeps_its_source_and_legacy_boundary_message() {
    let source = serde_json::from_str::<serde_json::Value>("{").unwrap_err();
    let expected = format!("invalid cargo-audit JSON: {source}");
    let error = CargoAuditParseError::InvalidJson(source);

    assert_eq!(error.to_string(), expected);
    assert!(error.source().is_some());
    assert!(matches!(error.into_mg_error(), MgError::Other(message) if message == expected));
}

#[test]
fn all_non_json_failures_keep_their_existing_messages() {
    let cases = [
        (
            CargoAuditParseError::UnrepresentablePackageName {
                name: "bad name".to_string(),
            },
            "cargo-audit finding for 'bad name' has a package name mgc cannot represent",
        ),
        (
            CargoAuditParseError::UnparseablePackageVersion {
                name: "serde".to_string(),
                version: "bad-version".to_string(),
            },
            "cargo-audit finding for 'serde' has an unparseable version 'bad-version'",
        ),
        (
            CargoAuditParseError::CountMismatch {
                count: 2,
                actual: 1,
            },
            "cargo-audit JSON inconsistent: count says 2 but list holds 1",
        ),
        (
            CargoAuditParseError::FoundListMismatch,
            "cargo-audit JSON inconsistent: found flag disagrees with list contents",
        ),
    ];

    for (error, expected) in cases {
        assert_eq!(error.to_string(), expected);
    }
}
