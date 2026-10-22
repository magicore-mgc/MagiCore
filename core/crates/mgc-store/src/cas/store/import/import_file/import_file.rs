//! CAS import owner implementation.
//! Triển khai owner import CAS.

use std::fs;
use std::path::Path;

use rayon::prelude::*;

use super::super::super::error::StoreError;
use super::super::super::export::sync_parent_directories;
use super::super::super::types::ContentStore;
use crate::cas::integrity::IntegrityHash;

impl ContentStore {
    pub fn import_file(&self, src: &Path) -> Result<IntegrityHash, StoreError> {
        self.import_file_with_exec(src, is_executable(src))
    }

    /// Import a regular file with an explicit executable bit. Large inputs
    /// are streamed into CAS so callers do not need to hold them in memory.
    /// (Nạp file thường với bit executable tường minh. File lớn được stream
    /// vào CAS để caller không phải giữ toàn bộ nội dung trong RAM.)
    pub fn import_file_with_exec(
        &self,
        src: &Path,
        executable: bool,
    ) -> Result<IntegrityHash, StoreError> {
        self.import_file_mode(src, executable, true, true)
    }

    /// Import independent files concurrently and sync each CAS shard once.
    /// Large source files are streamed with a bounded buffer.
    /// (Nạp file độc lập song song, sync mỗi shard CAS một lần; file lớn
    /// được stream với buffer có giới hạn.)
    pub fn import_files_with_exec_batch<'a, I>(
        &self,
        items: I,
    ) -> Result<Vec<IntegrityHash>, StoreError>
    where
        I: IntoIterator<Item = (&'a Path, bool)>,
    {
        self.import_files_with_exec_batch_mode(items, true, true)
    }

    /// Import package files whose verified archive can recreate CAS content
    /// after a crash. Writes remain isolated, hashed, and atomically published.
    /// (Nạp file package có thể dựng lại từ archive đã xác minh sau crash.
    /// File vẫn được cách ly, hash và publish nguyên tử.)
    pub fn import_rebuildable_files_with_exec_batch<'a, I>(
        &self,
        items: I,
    ) -> Result<Vec<IntegrityHash>, StoreError>
    where
        I: IntoIterator<Item = (&'a Path, bool)>,
    {
        self.import_files_with_exec_batch_mode(items, false, false)
    }

    fn import_files_with_exec_batch_mode<'a, I>(
        &self,
        items: I,
        sync_file_data: bool,
        sync_cas_directories: bool,
    ) -> Result<Vec<IntegrityHash>, StoreError>
    where
        I: IntoIterator<Item = (&'a Path, bool)>,
    {
        let items: Vec<_> = items.into_iter().collect();
        let imported: Result<Vec<_>, _> = items
            .par_iter()
            .map(|(src, executable)| self.import_file_mode(src, *executable, sync_file_data, false))
            .collect();
        if sync_cas_directories && let Ok(hashes) = &imported {
            let sync_dirs = hashes
                .iter()
                .filter_map(|hash| hash.cas_path(&self.root).parent().map(Path::to_path_buf))
                .collect();
            sync_parent_directories(sync_dirs)?;
        }
        imported
    }

    fn import_file_mode(
        &self,
        src: &Path,
        executable: bool,
        sync_file_data: bool,
        sync_parent_directory: bool,
    ) -> Result<IntegrityHash, StoreError> {
        super::import_file_mode::import_file_mode(
            self,
            src,
            executable,
            sync_file_data,
            sync_parent_directory,
        )
    }
}

fn is_executable(path: &Path) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(meta) = fs::metadata(path) {
            return (meta.permissions().mode() & 0o111) != 0;
        }
    }
    #[cfg(not(unix))]
    {
        let _ = path;
    }
    false
}
