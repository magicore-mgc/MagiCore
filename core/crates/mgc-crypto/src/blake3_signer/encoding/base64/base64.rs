//! Standard base64 codec.
//! Codec base64 chuẩn.

use crate::blake3_signer::error::EncodingError;

pub(in crate::blake3_signer) fn encode(bytes: &[u8]) -> String {
    use ::base64::Engine;
    ::base64::engine::general_purpose::STANDARD.encode(bytes)
}

pub(in crate::blake3_signer) fn decode(value: &str) -> Result<Vec<u8>, EncodingError> {
    use ::base64::Engine;
    ::base64::engine::general_purpose::STANDARD
        .decode(value)
        .map_err(|error| EncodingError::InvalidBase64(error.to_string()))
}
