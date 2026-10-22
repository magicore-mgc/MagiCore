//! Preserve the inline signature wire shape.
//! Giữ nguyên wire shape chữ ký nhúng.

#![allow(clippy::unwrap_used)]

use super::*;

#[test]
fn signature_block_roundtrips_all_signed_fields() {
    let block = SignatureBlock {
        algorithm: "ed25519".to_string(),
        key_id: "0123456789abcdef".to_string(),
        public_key: "cHVibGljLWtleQ==".to_string(),
        digest: "blake3-ZGlnZXN0".to_string(),
        signed_at: "2026-10-17T00:00:00Z".to_string(),
        signature: "ed25519-c2ln".to_string(),
    };
    let encoded = serde_json::to_string(&block).expect("test operation must succeed");
    let decoded: SignatureBlock =
        serde_json::from_str(&encoded).expect("test operation must succeed");
    assert_eq!(decoded, block);
}
