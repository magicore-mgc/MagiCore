//! Base64 STANDARD conversion — Chuyển đổi Base64 STANDARD.
//! Keep the dependency codec and typed decoding failure — Giữ codec dependency và lỗi decode có kiểu.

use crate::ed25519_signer::error::Ed25519Error;

pub(in crate::ed25519_signer) fn encode(bytes: &[u8]) -> String {
    use ::base64::Engine;
    ::base64::engine::general_purpose::STANDARD.encode(bytes)
}

pub(in crate::ed25519_signer) fn decode(value: &str) -> Result<Vec<u8>, Ed25519Error> {
    use ::base64::Engine;
    ::base64::engine::general_purpose::STANDARD
        .decode(value)
        .map_err(Ed25519Error::Base64Decode)
}
