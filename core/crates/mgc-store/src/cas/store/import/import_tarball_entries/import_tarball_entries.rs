//! CAS import owner implementation.
//! Triển khai owner import CAS.

use super::super::super::error::StoreError;
use super::super::super::types::ContentStore;
use crate::cas::integrity::{self, IntegrityHash, TarballEntry};

impl ContentStore {
    /// Import tarball entries into the CAS atomically (P0-3).
    /// Every entry goes through the SAME primitive as `write_bytes_with_hash`:
    /// unique temp (same CAS filesystem) → write + fsync → verify → atomic
    /// publish → if the rename race was lost, the winner (fully written,
    /// content-addressed) is re-verified instead of failing with
    /// AlreadyExists. Readers can never observe a half-written blob at the
    /// final CAS path.
    ///
    /// Import entry tarball vào CAS nguyên tử (P0-3): mọi entry đi qua CÙNG
    /// nguyên thủy với `write_bytes_with_hash` — temp duy nhất (cùng
    /// filesystem CAS) → ghi + fsync → verify → publish nguyên tử → thua
    /// race thì re-verify winner (đã ghi xong, content-addressed) thay vì
    /// fail AlreadyExists. Reader không bao giờ thấy blob nửa vời ở path
    /// CAS cuối.
    pub fn import_tarball_entries(
        &self,
        entries: Vec<TarballEntry>,
    ) -> Result<Vec<IntegrityHash>, StoreError> {
        let mut imported = Vec::with_capacity(entries.len());
        for entry in entries {
            let hash = integrity::IntegrityHash::from_bytes(&entry.data, entry.executable);
            self.write_bytes_with_hash(&entry.data, &hash, entry.executable)?;
            imported.push(hash);
        }
        Ok(imported)
    }
}
