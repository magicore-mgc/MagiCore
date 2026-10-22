// Bun parser error contract tests — test hợp đồng lỗi parser Bun.
// Verify stable display text, shared error mapping, and the serde source chain.
// Xác minh nội dung hiển thị ổn định, mapping lỗi dùng chung và source chain serde.

use super::BunLockError;
use mgc_types::MgError;
use std::error::Error;

#[test]
fn typed_errors_keep_legacy_messages_and_serde_source() {
    let array_error = BunLockError::PackageEntryNotArray("pkg".to_string());
    assert_eq!(
        array_error.to_string(),
        "bun.lock package 'pkg' entry must be an array"
    );
    let MgError::Other(message) = MgError::from(array_error) else {
        panic!("Bun parse errors must retain the Other variant");
    };
    assert_eq!(message, "bun.lock package 'pkg' entry must be an array");

    let source = serde_json::from_str::<serde_json::Value>("{")
        .expect_err("malformed JSON should produce a serde error");
    let json_error = BunLockError::InvalidJson(source);
    assert!(json_error.source().is_some());
    assert!(json_error.to_string().starts_with("invalid bun.lock: "));
}
