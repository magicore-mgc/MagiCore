//! Compatibility audit error source and redaction tests.
//! Test source và redaction lỗi audit compatibility.

use super::*;
use std::error::Error;

#[test]
fn write_error_keeps_its_cause_without_echoing_raw_input() {
    let error = CompatAuditError::WriteFailed(anyhow::Error::msg("disk denied"));
    assert_eq!(error.to_string(), "compat audit event could not be written");
    assert_eq!(
        error.source().map(ToString::to_string).as_deref(),
        Some("disk denied")
    );
}

#[test]
fn poisoned_dedup_state_has_a_stable_category() {
    let error = CompatAuditError::DedupStatePoisoned;
    assert_eq!(error.to_string(), "compat audit process set was poisoned");
    assert!(error.source().is_none());
}
