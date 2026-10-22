//! Bounded-memory streaming digest.
//! Tính digest streaming với bộ nhớ giới hạn.
//! Digest streaming với bộ nhớ giới hạn.

use crate::{CryptoResult, blake3_signer::types::Blake3Hash};
use std::io::Read;

pub(in crate::blake3_signer) fn hash_file(path: &std::path::Path) -> CryptoResult<Blake3Hash> {
    let mut file = std::fs::File::open(path)?;
    let mut hasher = blake3::Hasher::new();
    let mut buffer = [0u8; 8192];
    loop {
        let read = file.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    Ok(Blake3Hash(*hasher.finalize().as_bytes()))
}
