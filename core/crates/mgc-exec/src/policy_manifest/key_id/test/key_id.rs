use crate::policy_manifest::test::support::test_key;
use crate::policy_manifest::{is_canonical_key_id_hex, key_id_hex};

// Key-id hex is the value supplied through user-level trust configuration.
// (Key-id hex là giá trị trong cấu hình trust cấp user.)
#[test]
fn key_id_hex_format() {
    let (_, key_id) = test_key();
    let hex = key_id_hex(&key_id).expect("valid test fixture");
    assert_eq!(hex.len(), 16);
    assert!(hex.chars().all(|c| c.is_ascii_hexdigit()));
    assert!(key_id_hex("!!!not-base64!!!").is_none());
}

#[test]
fn revoked_key_id_shape_accepts_only_lowercase_sixteen_digit_hex() {
    assert!(is_canonical_key_id_hex("0123456789abcdef"));
    assert!(!is_canonical_key_id_hex("ABCDEF0123456789"));
    assert!(!is_canonical_key_id_hex("0123456789abcde"));
}
