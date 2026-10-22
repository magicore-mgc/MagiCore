//! Streams a file through BLAKE3 after rejecting links and non-regular paths.
//! Stream file qua BLAKE3 sau khi từ chối symlink và path không phải file thường.

use std::fs;
use std::io::{BufReader, Read};
use std::path::Path;

use super::super::error::StoreError;
use super::super::types::ContentStore;
#[cfg(unix)]
use super::file_identity::file_identity;
use super::open_file::open_regular_file;
use crate::cas::integrity::IntegrityHash;

impl ContentStore {
    pub fn verify(&self, path: &Path) -> Result<IntegrityHash, StoreError> {
        let mut file = open_regular_file(path, "path is a symlink, refusing to verify")?;
        verify_open_file(&mut file, path)
    }
}

pub(in crate::cas::store) fn verify_open_file(
    file: &mut fs::File,
    path: &Path,
) -> Result<IntegrityHash, StoreError> {
    #[cfg(unix)]
    let before_identity = {
        let metadata = file.metadata().map_err(|error| StoreError::Io {
            path: path.to_path_buf(),
            msg: error.to_string(),
        })?;
        file_identity(&metadata)
    };
    // Streaming hash (P1-A, Tech Lead vòng-7): `fs::read` loaded the
    // WHOLE file into RAM — memory grew linearly with blob size, which
    // rules out AI-model/game-asset/monorepo-scale blobs. The fixed
    // 64-KiB buffer keeps RSS flat regardless of file size; the digest
    // is byte-identical to the old in-memory form (BLAKE3 over the
    // same bytes).
    // (Hash streaming (P1-A): `fs::read` load TOÀN BỘ file vào RAM —
    // memory tăng tuyến tính theo kích thước blob, vô hiệu hóa blob
    // cỡ AI-model/game-asset/monorepo. Buffer cố định 64-KiB giữ RSS
    // phẳng bất kể kích thước file; digest giống hệt byte với dạng
    // in-memory cũ (BLAKE3 trên cùng bytes).)
    let mut reader = BufReader::new(&mut *file);
    let mut hasher = blake3::Hasher::new();
    let mut buf = [0u8; 65536];
    loop {
        let n = reader.read(&mut buf).map_err(|e| StoreError::Io {
            path: path.to_path_buf(),
            msg: format!("read for verify failed: {e}"),
        })?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }
    drop(reader);
    let hex = hasher.finalize().to_hex().to_string();
    #[cfg(unix)]
    {
        let after_metadata = file.metadata().map_err(|error| StoreError::Io {
            path: path.to_path_buf(),
            msg: error.to_string(),
        })?;
        let after_identity = file_identity(&after_metadata);
        if before_identity != after_identity {
            return Err(StoreError::Io {
                path: path.to_path_buf(),
                msg: "file changed while it was being verified".to_string(),
            });
        }
    }
    // Behavior-preserving: the old form always hashed with
    // `executable: false` — keep that exact contract (callers compare
    // `as_hex()` which carries no exec bit, and changing it here would
    // silently alter IntegrityHash equality semantics).
    // (Giữ nguyên hành vi: dạng cũ luôn hash với `executable: false` —
    // giữ đúng hợp đồng đó (caller so `as_hex()` không chứa exec bit,
    // và đổi ở đây sẽ âm thầm thay đổi semantics bằng-equal của
    // IntegrityHash).)
    IntegrityHash::from_hash_trusted(&hex, false).map_err(|e| StoreError::InvalidHash(e.input))
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
