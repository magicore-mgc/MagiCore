//! Boundary tests for bounded registry signature responses.
//! Test biên giới hạn response chữ ký registry.

use super::bundle_reader::MAX_SIGNATURE_BUNDLE_BYTES;
use super::bundle_reader::ensure_response_size;
use super::error::BundleReadError;

#[test]
fn accepts_a_response_at_the_limit() {
    assert!(ensure_response_size(Some(MAX_SIGNATURE_BUNDLE_BYTES as u64), 0, 0).is_ok());
    assert!(ensure_response_size(None, MAX_SIGNATURE_BUNDLE_BYTES - 1, 1).is_ok());
}

#[test]
fn rejects_declared_and_streamed_responses_over_the_limit() {
    assert!(ensure_response_size(Some(MAX_SIGNATURE_BUNDLE_BYTES as u64 + 1), 0, 0).is_err());
    assert!(ensure_response_size(None, MAX_SIGNATURE_BUNDLE_BYTES, 1).is_err());
}

#[test]
fn rejects_an_already_oversized_accumulated_body() {
    assert!(ensure_response_size(None, MAX_SIGNATURE_BUNDLE_BYTES + 1, 0).is_err());
}

#[test]
fn invalid_response_keeps_the_cli_message_on_top_and_retains_the_source() {
    let error = BundleReadError::Invalid(anyhow::anyhow!("source detail")).into_cli();
    assert_eq!(
        error.to_string(),
        "registry returned an invalid trusted signatures response"
    );
    assert!(format!("{error:#}").contains("source detail"));
}
