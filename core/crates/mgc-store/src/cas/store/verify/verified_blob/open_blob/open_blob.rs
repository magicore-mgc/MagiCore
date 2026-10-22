//! Binds memo checks and content hashing to one already-open CAS file.
//! Gắn memo và hash nội dung vào cùng file CAS đã mở.

use std::fs::File;
use std::io::{Seek, SeekFrom};
use std::path::Path;

use crate::cas::integrity::IntegrityHash;
use crate::cas::store::StoreError;
use crate::cas::store::types::{ContentStore, VerifiedBlobKey};
use crate::cas::store::verify::file_identity::file_identity;
use crate::cas::store::verify::quarantine::quarantine_blob;
use crate::cas::store::verify::verify_open_file;

impl ContentStore {
    pub(in crate::cas::store) fn verify_opened_blob(
        &self,
        hash: &IntegrityHash,
        path: &Path,
        mut file: File,
    ) -> Result<File, StoreError> {
        file.seek(SeekFrom::Start(0))
            .map_err(|error| StoreError::Io {
                path: path.to_path_buf(),
                msg: error.to_string(),
            })?;
        let memo_key = VerifiedBlobKey {
            digest: hash.as_hex().to_string(),
            executable: hash.is_executable(),
        };
        let metadata = file.metadata().map_err(|error| StoreError::Io {
            path: path.to_path_buf(),
            msg: error.to_string(),
        })?;
        let identity = file_identity(&metadata);

        #[cfg(unix)]
        if let Some(id) = identity
            && let Some(cached) = self.verified.lock().get(&memo_key)
            && *cached == id
        {
            self.memo_counters
                .hits
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            return Ok(file);
        }
        #[cfg(not(unix))]
        let _ = memo_key;
        self.memo_miss();

        let actual = match verify_open_file(&mut file, path) {
            Ok(actual) => actual,
            Err(StoreError::NotFound(_)) => {
                return Err(StoreError::NotFound(hash.as_hex().to_string()));
            }
            Err(error) => return Err(error),
        };
        if actual.as_hex() != hash.as_hex() {
            #[cfg(unix)]
            self.verified.lock().pop(&memo_key);
            // Evidence capture is best-effort; preserve the primary mismatch.
            // Lưu evidence là best-effort; giữ lỗi mismatch gốc.
            let _ = quarantine_blob(self.root(), path, file, hash.as_hex(), actual.as_hex());
            return Err(StoreError::HashMismatch {
                expected: hash.as_hex().to_string(),
                actual: actual.as_hex().to_string(),
            });
        }

        #[cfg(unix)]
        if let Some(id) = identity {
            let mut cache = self.verified.lock();
            let was_full = cache.len() == cache.cap().get();
            let key_existed = cache.contains(&memo_key);
            cache.push(memo_key, id);
            if was_full && !key_existed {
                self.memo_counters
                    .evictions
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }
        }
        #[cfg(not(unix))]
        let _ = identity;

        file.seek(SeekFrom::Start(0))
            .map_err(|error| StoreError::Io {
                path: path.to_path_buf(),
                msg: error.to_string(),
            })?;
        Ok(file)
    }

    fn memo_miss(&self) {
        #[cfg(unix)]
        self.memo_counters
            .misses
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
