//! Bounded hexadecimal codec for 32-byte digests.
//! Codec hex giới hạn cho digest 32 byte.
//! Codec hex có giới hạn cho digest 32 byte.

use crate::blake3_signer::error::EncodingError;

pub(in crate::blake3_signer) fn encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}

pub(in crate::blake3_signer) fn decode(value: &str) -> Result<Vec<u8>, EncodingError> {
    const MAX_HEX_LEN: usize = 128;
    if value.len() > MAX_HEX_LEN {
        return Err(EncodingError::InvalidHex(format!(
            "hex string too long: {} > {MAX_HEX_LEN}",
            value.len()
        )));
    }
    if !value.len().is_multiple_of(2) {
        return Err(EncodingError::InvalidHex("odd length".to_string()));
    }
    let mut output = Vec::with_capacity(value.len() / 2);
    for [high_byte, low_byte] in value.as_bytes().as_chunks::<2>().0 {
        let high = decode_nibble(*high_byte)?;
        let low = decode_nibble(*low_byte)?;
        output.push((high << 4) | low);
    }
    Ok(output)
}

fn decode_nibble(byte: u8) -> Result<u8, EncodingError> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        b'A'..=b'F' => Ok(byte - b'A' + 10),
        _ => Err(EncodingError::InvalidHex(
            "invalid hex: non-hexadecimal byte".to_string(),
        )),
    }
}
