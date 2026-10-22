//! Staged Hash owner.
//! Phụ trách staged hash trong luồng export CAS.

use std::fs;
use std::io::{BufReader, Read};
use std::path::Path;

use crate::cas::store::StoreError;

/// Hash a file's bytes (BLAKE3, streaming — no full in-memory load).
/// Used by export to verify the STAGED artifact end-to-end before publish.
///
/// Hash bytes của file (BLAKE3, streaming — không load toàn bộ vào memory).
/// Dùng bởi export để verify artifact STAGED end-to-end trước khi publish.
pub(in crate::cas::store::export) fn hash_file(path: &Path) -> Result<String, StoreError> {
    let file = fs::File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = blake3::Hasher::new();
    let mut buf = [0u8; 65536];
    loop {
        let n = reader.read(&mut buf).map_err(|e| StoreError::Io {
            path: path.to_path_buf(),
            msg: format!("read for hash failed: {e}"),
        })?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    Ok(hasher.finalize().to_hex().to_string())
}
