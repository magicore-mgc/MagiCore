// Public invalid-hash diagnostic compatibility test.
// Test tương thích diagnostic InvalidHashError công khai.
use super::super::InvalidHashError;

#[test]
fn invalid_hash_error_keeps_legacy_reason_and_display() {
    let error = InvalidHashError {
        input: "short".to_string(),
    };

    assert_eq!(error.reason(), "expected 64 hex characters");
    assert_eq!(
        error.to_string(),
        "invalid blake3 digest 'short': expected 64 hex characters"
    );
}
