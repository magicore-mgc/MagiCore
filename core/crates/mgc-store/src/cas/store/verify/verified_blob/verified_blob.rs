//! Opens and verifies a stored blob before discarding its handle.
//! Mở và xác minh blob đã lưu trước khi đóng handle.

use crate::cas::integrity::IntegrityHash;
use crate::cas::store::StoreError;
use crate::cas::store::types::ContentStore;
use crate::cas::store::verify::open_regular_file;

impl ContentStore {
    pub(in crate::cas::store) fn open_verified_blob(
        &self,
        hash: &IntegrityHash,
        path: &std::path::Path,
    ) -> Result<std::fs::File, StoreError> {
        let file = open_regular_file(path, "path is a symlink, refusing to fingerprint")?;
        self.verify_opened_blob(hash, path, file)
    }

    pub(in crate::cas::store) fn verify_stored_blob(
        &self,
        hash: &IntegrityHash,
    ) -> Result<(), StoreError> {
        let path = hash.cas_path(&self.root);
        self.open_verified_blob(hash, &path).map(drop)
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
