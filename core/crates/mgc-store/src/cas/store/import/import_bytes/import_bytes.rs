//! CAS import owner implementation.
//! Triển khai owner import CAS.

use std::path::{Path, PathBuf};

use rayon::prelude::*;

use super::super::super::error::StoreError;
use super::super::super::export::sync_parent_directories;
use super::super::super::types::ContentStore;
use crate::cas::integrity::IntegrityHash;

impl ContentStore {
    pub fn import_bytes(&self, data: &[u8]) -> Result<IntegrityHash, StoreError> {
        self.import_bytes_with_exec(data, false)
    }

    pub fn import_bytes_with_exec(
        &self,
        data: &[u8],
        executable: bool,
    ) -> Result<IntegrityHash, StoreError> {
        let hash = IntegrityHash::from_bytes(data, executable);
        self.write_generated_bytes_with_hash_mode(data, &hash, executable, true, true)
    }

    /// Import independent blobs concurrently, then sync each CAS shard once.
    /// File contents are still fsynced and verified before their atomic publish.
    /// (Nạp blob độc lập song song, rồi sync mỗi shard CAS một lần; dữ liệu
    /// từng file vẫn được fsync và xác minh trước khi publish nguyên tử.)
    pub fn import_bytes_with_exec_batch<'a, I>(
        &self,
        items: I,
    ) -> Result<Vec<IntegrityHash>, StoreError>
    where
        I: IntoIterator<Item = (&'a [u8], bool)>,
    {
        self.import_bytes_with_exec_batch_mode(items, true, true)
    }

    /// Import package-file blobs whose source archive and extracted package
    /// can be recreated and integrity-checked after a crash. Hashing, temp-file
    /// isolation, atomic rename, and later read-time verification remain active;
    /// callers must retain the verified archive needed to rebuild missing blobs.
    /// (Nạp blob file package có thể dựng lại và xác minh integrity từ archive
    /// sau crash. Vẫn hash, cách ly temp, rename nguyên tử và verify lúc đọc;
    /// caller phải giữ archive đã xác minh để dựng lại blob thiếu.)
    pub fn import_rebuildable_bytes_with_exec_batch<'a, I>(
        &self,
        items: I,
    ) -> Result<Vec<IntegrityHash>, StoreError>
    where
        I: IntoIterator<Item = (&'a [u8], bool)>,
    {
        self.import_bytes_with_exec_batch_mode(items, false, false)
    }

    fn import_bytes_with_exec_batch_mode<'a, I>(
        &self,
        items: I,
        sync_file_data: bool,
        sync_cas_directories: bool,
    ) -> Result<Vec<IntegrityHash>, StoreError>
    where
        I: IntoIterator<Item = (&'a [u8], bool)>,
    {
        let prepared: Vec<_> = items
            .into_iter()
            .map(|(data, executable)| {
                (
                    IntegrityHash::from_bytes(data, executable),
                    data,
                    executable,
                )
            })
            .collect();
        let sync_dirs: std::collections::BTreeSet<PathBuf> = if sync_cas_directories {
            prepared
                .iter()
                .filter_map(|(hash, _, _)| {
                    hash.cas_path(&self.root).parent().map(Path::to_path_buf)
                })
                .collect()
        } else {
            std::collections::BTreeSet::new()
        };

        let imported: Result<Vec<_>, _> = prepared
            .par_iter()
            .map(|(hash, data, executable)| {
                self.write_generated_bytes_with_hash_mode(
                    data,
                    hash,
                    *executable,
                    false,
                    sync_file_data,
                )
            })
            .collect();

        let sync_result = if sync_cas_directories {
            sync_parent_directories(sync_dirs)
        } else {
            Ok(())
        };
        sync_result?;
        imported
    }

    pub fn import_bytes_with_hash(
        &self,
        data: &[u8],
        hash_hex: &str,
        executable: bool,
    ) -> Result<IntegrityHash, StoreError> {
        let hash = IntegrityHash::from_hash_str(hash_hex, executable)?;
        self.write_bytes_with_hash(data, &hash, executable)
    }
}
