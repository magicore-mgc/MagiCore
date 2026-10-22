//! Reject malformed trust IDs and normalize accepted hex IDs.
//! Loại ID trust sai và chuẩn hóa hex ID hợp lệ.

#![allow(clippy::unwrap_used)]

use super::parse_trusted_policy_key_ids;

#[test]
fn parses_only_sixteen_digit_hex_ids_and_normalizes_case() {
    assert_eq!(
        parse_trusted_policy_key_ids(
            " 0123456789ABCDEF, invalid, deadbeefdeadbeef0, 0123456789abcdef "
        ),
        vec![
            "0123456789abcdef".to_string(),
            "0123456789abcdef".to_string()
        ]
    );
}
