// Deno parser error contract tests — test hợp đồng lỗi parser Deno.
// Verify stable display text, shared error mapping, and the serde source chain.
// Xác minh nội dung hiển thị ổn định, mapping lỗi dùng chung và source chain serde.

use super::DenoLockError;
use mgc_types::MgError;
use std::error::Error;

#[test]
fn typed_errors_keep_legacy_messages_and_serde_source() {
    let pin_error = DenoLockError::NpmPinWithoutVersion("lodash".to_string());
    assert_eq!(
        pin_error.to_string(),
        "deno.lock npm pin 'lodash' must be name@version"
    );
    let MgError::Other(message) = MgError::from(pin_error) else {
        panic!("Deno parse errors must retain the Other variant");
    };
    assert_eq!(message, "deno.lock npm pin 'lodash' must be name@version");

    let source =
        serde_json::from_str::<serde_json::Value>("{").expect_err("reject malformed JSON fixture");
    let json_error = DenoLockError::InvalidJson(source);
    assert!(json_error.source().is_some());
    assert!(json_error.to_string().starts_with("invalid deno.lock: "));
}
