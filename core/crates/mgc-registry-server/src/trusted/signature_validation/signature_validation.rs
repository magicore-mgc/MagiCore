//! Decodes the shared signature wire format and validates npm SHA-512/PyPI SHA-256 inputs.
//! Giải mã wire signature dùng chung và kiểm tra đầu vào npm SHA-512/PyPI SHA-256.

use super::super::prelude::*;

/// Decode a signature using the wire format shared by registry and CLI.
/// Giải mã chữ ký theo wire format dùng chung giữa registry và CLI.
pub fn decode_signature(value: &str) -> Result<Ed25519Signature> {
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(value)
        .context("decode provenance signature")?;
    Ok(Ed25519Signature(bytes))
}

/// Accept only npm's canonical SHA-512 integrity representation.
/// Chỉ nhận định dạng integrity SHA-512 chuẩn của npm.
pub fn is_valid_sha512_integrity(digest: &str) -> bool {
    let Some(encoded) = digest.strip_prefix("sha512-") else {
        return false;
    };
    base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .is_ok_and(|bytes| bytes.len() == 64)
}

/// Accept one canonical 32-byte SHA-256 hex digest.
/// Chỉ nhận chuỗi hex SHA-256 đúng 32 byte.
pub fn is_valid_sha256_hex(digest: &str) -> bool {
    digest.len() == 64 && digest.bytes().all(|byte| byte.is_ascii_hexdigit())
}
