//! Error category and redaction tests.
//! Test loại lỗi và redaction.

use super::*;

#[test]
fn authentication_errors_are_typed_and_redacted() {
    let error = NpmProbeAuthenticationError::ImageDigestMismatch;
    assert_eq!(
        error.to_string(),
        "process image hash does not match the running MGC executable"
    );
    assert!(!error.to_string().contains("/"));
    assert_eq!(
        NpmProbeAuthenticationError::ImagePathUnavailable,
        NpmProbeAuthenticationError::ImagePathUnavailable
    );
}
