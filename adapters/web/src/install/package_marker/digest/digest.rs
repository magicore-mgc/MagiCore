//! In-memory and streaming SHA-256 helpers.
//! Hàm SHA-256 cho dữ liệu nhớ và luồng file.

use std::io::Read;
use std::path::Path;

use mgc_types::{MgError, MgResult};
use sha2::{Digest, Sha256};

pub(crate) const SHA256_READ_BUFFER_BYTES: usize = 64 * 1024;

pub fn compute_sha256_hex(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    hex::encode(hasher.finalize())
}

pub fn compute_sha256_hex_from_path(path: &Path) -> MgResult<String> {
    let mut file = std::fs::File::open(path).map_err(|err| {
        MgError::Other(format!(
            "failed to open '{}' for sha256: {}",
            path.display(),
            err
        ))
    })?;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; SHA256_READ_BUFFER_BYTES];
    loop {
        let bytes_read = file.read(&mut buffer).map_err(|err| {
            MgError::Other(format!(
                "failed to hash '{}' with sha256: {}",
                path.display(),
                err
            ))
        })?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
    Ok(hex::encode(hasher.finalize()))
}

pub fn compute_sha256_hex_from_reader<R: Read>(mut reader: R) -> MgResult<String> {
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; SHA256_READ_BUFFER_BYTES];
    loop {
        let bytes_read = reader.read(&mut buffer).map_err(|err| {
            MgError::Other(format!("failed to hash tarball reader with sha256: {err}"))
        })?;
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }
    Ok(hex::encode(hasher.finalize()))
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
