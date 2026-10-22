#![allow(clippy::unwrap_used)]
//! Public SBOM error contract tests — Test contract lỗi SBOM public

use super::super::*;
use std::error::Error;

#[test]
fn keeps_invalid_lockfile_display_text() {
    let error = SbomError::InvalidLockfile("bad graph".into());
    assert_eq!(error.to_string(), "Invalid lockfile: bad graph");
}

#[test]
fn conversion_keeps_serialization_variant_and_source() {
    let source = serde_json::from_str::<serde_json::Value>("{").unwrap_err();
    let error = SbomError::from(source);

    assert!(matches!(&error, SbomError::SerializationError(_)));
    assert!(error.source().is_some());
}

#[test]
fn result_alias_uses_the_public_error_type() {
    fn parse_invalid_json() -> SbomResult<serde_json::Value> {
        Ok(serde_json::from_str("{")?)
    }

    assert!(matches!(
        parse_invalid_json(),
        Err(SbomError::SerializationError(_))
    ));
}
