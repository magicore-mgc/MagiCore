//! Streaming BLAKE3 hashing for cache files.
//! Băm BLAKE3 theo luồng cho file cache.

use crate::package_cache::package_cache::PackageCache;
use anyhow::Result;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Hash input size keeps memory constant for large archives.
/// Kích thước đọc giữ bộ nhớ cố định với archive lớn.
const INTEGRITY_BUFFER_BYTES: usize = 64 * 1024;

impl PackageCache {
    /// Compute a BLAKE3 integrity string without loading the file into memory.
    /// Tính chuỗi integrity BLAKE3 mà không nạp cả file vào bộ nhớ.
    pub fn compute_integrity(&self, path: &Path) -> Result<String> {
        hash_reader(File::open(path)?)
    }

    /// Hash a cache-owned file without following its final symlink.
    /// Băm file thuộc cache mà không đi theo symlink cuối.
    pub(in crate::package_cache) fn compute_integrity_no_follow(
        &self,
        path: &Path,
    ) -> Result<String> {
        hash_reader(mgc_platform::open_regular_file_no_follow(path)?)
    }
}

fn hash_reader(mut file: impl Read) -> Result<String> {
    let mut hasher = blake3::Hasher::new();
    let mut buffer = [0u8; INTEGRITY_BUFFER_BYTES];
    loop {
        let read = file.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    Ok(format!("blake3:{}", hasher.finalize().to_hex()))
}
