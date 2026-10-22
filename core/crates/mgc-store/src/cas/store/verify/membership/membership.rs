//! Exposes CAS membership checks and removal with memo invalidation.
//! Cung cấp kiểm tra membership CAS và xóa kèm vô hiệu hóa memo.

use crate::cas::integrity::IntegrityHash;
use crate::cas::store::StoreError;
use crate::cas::store::types::{ContentStore, VerifiedBlobKey};
use std::fs;

impl ContentStore {
    pub fn contains(&self, hash: &IntegrityHash) -> bool {
        hash.cas_path(&self.root).exists()
    }

    pub fn remove(&self, hash: &IntegrityHash) -> Result<(), StoreError> {
        let path = hash.cas_path(&self.root);
        if path.exists() {
            fs::remove_file(&path)?;
        }
        // Memo invalidation on delete (P1-2 audit vòng-4): the entry vouched
        // for a file that no longer exists — it must not survive the blob.
        // (Vô hiệu hóa memo khi xóa (P1-2 audit vòng-4): entry bảo chứng cho
        // file không còn tồn tại — nó không được sống sót qua blob.)
        #[cfg(unix)]
        self.verified.lock().pop(&VerifiedBlobKey {
            digest: hash.as_hex().to_string(),
            executable: hash.is_executable(),
        });
        Ok(())
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
