//! Covers forward-compatible signature sidecar parsing.
//! Kiểm tra parse signature sidecar tương thích mở rộng.

use super::super::SignatureFile;

#[test]
fn ignores_unknown_sidecar_fields() {
    let input = r#"
lockfile_hash = "blake3-test"
signature = "ed25519-test"
key_id = "test-key"
signed_at = "2026-10-17T00:00:00Z"
future_field = "ignored"
"#;

    let signature: SignatureFile = input.parse().expect("test operation must succeed");
    assert_eq!(signature.lockfile_hash, "blake3-test");
    assert_eq!(signature.signature, "ed25519-test");
    assert_eq!(signature.key_id, "test-key");
}
