//! Release identity validation tests.
//! Test kiểm tra identity release.

use super::super::*;

#[test]
fn release_identity_must_match_requested_version() {
    let info = parse_release_identity(
        "example.test/pkg",
        "1.2.3",
        r#"{"Version":"v1.2.3","Time":"2024-01-02T03:04:05Z"}"#,
    )
    .expect("test operation must succeed");
    assert_eq!(info.version, "v1.2.3");
    assert_eq!(info.time, "2024-01-02T03:04:05Z");

    let error = parse_release_identity("example.test/pkg", "1.2.3", r#"{"Version":"v9.9.9"}"#)
        .expect_err("test operation must fail");
    assert!(matches!(
        error,
        MgError::Integrity(message) if message.contains("requested example.test/pkg@1.2.3")
    ));
}
