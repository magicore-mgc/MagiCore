//! CycloneDX hash to SPDX checksum projection — Chiếu hash CycloneDX sang checksum SPDX

use super::super::SpdxChecksum;
use crate::cyclonedx::Hash;

pub(in crate::spdx) fn project_checksum(hash: &Hash) -> Option<SpdxChecksum> {
    let normalized = hash
        .alg
        .chars()
        .filter(|character| *character != '-')
        .map(|character| character.to_ascii_uppercase())
        .collect::<String>();
    let (algorithm, hex_length) = match normalized.as_str() {
        "SHA1" => ("SHA1", 40),
        "SHA224" => ("SHA224", 56),
        "SHA256" => ("SHA256", 64),
        "SHA384" => ("SHA384", 96),
        "SHA512" => ("SHA512", 128),
        "SHA3256" => ("SHA3-256", 64),
        "SHA3384" => ("SHA3-384", 96),
        "SHA3512" => ("SHA3-512", 128),
        "BLAKE3" => ("BLAKE3", 64),
        "BLAKE2B256" => ("BLAKE2b-256", 64),
        "BLAKE2B384" => ("BLAKE2b-384", 96),
        "BLAKE2B512" => ("BLAKE2b-512", 128),
        "MD5" => ("MD5", 32),
        "ADLER32" => ("ADLER32", 8),
        _ => return None,
    };
    if hash.content.len() != hex_length
        || !hash.content.bytes().all(|byte| byte.is_ascii_hexdigit())
    {
        return None;
    }
    Some(SpdxChecksum {
        algorithm,
        checksum_value: hash.content.to_ascii_lowercase(),
    })
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
