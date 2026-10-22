//! Error-contract tests for manifest delta conversion.
//! Test contract lỗi khi chuyển đổi manifest delta.

use super::super::error::ManifestDeltaError;
use std::error::Error as StdError;

#[test]
fn invalid_range_error_keeps_the_original_message_and_source() {
    let error = ManifestDeltaError::from(mgc_types::MgError::InvalidVersionRange(String::new()));

    assert_eq!(error.to_string(), "invalid version range: ");
    assert_eq!(
        StdError::source(&error)
            .expect("wrapped package type error")
            .to_string(),
        "invalid version range: "
    );
}
