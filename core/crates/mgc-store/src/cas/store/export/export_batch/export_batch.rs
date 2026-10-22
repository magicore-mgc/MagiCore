//! Export Batch owner.
//! Phụ trách export batch trong luồng export CAS.

use std::path::{Path, PathBuf};

use rayon::prelude::*;

use crate::cas::integrity::IntegrityHash;
use crate::cas::store::{ContentStore, StoreError};

use super::super::directory_sync::sync_parent_directories;

impl ContentStore {
    /// Export independent files concurrently and sync each changed directory once.
    /// Existing single-file integrity, symlink, mode, and atomic-rename checks remain active.
    /// (Xuất các bản sao độc lập song song và sync mỗi thư mục thay đổi một lần;
    /// toàn bộ kiểm tra integrity, symlink, mode và atomic rename vẫn được giữ.)
    pub fn export_batch_to<I>(&self, items: I) -> Result<(), StoreError>
    where
        I: IntoIterator<Item = (IntegrityHash, PathBuf)>,
    {
        self.export_batch_to_mode(items, true)
    }

    /// Export verified files into a rebuildable package-cache root.
    /// Per-file validation and atomic publication remain active, but destination
    /// directories are not synced. Callers must validate and rebuild the root
    /// from its verified archive before reuse after a crash.
    /// (Xuất file đã xác minh vào root cache package có thể dựng lại. Vẫn giữ
    /// kiểm tra từng file và publish nguyên tử nhưng không sync thư mục đích.
    /// Caller phải xác minh và dựng lại root từ archive đã xác minh trước khi
    /// dùng lại sau crash.)
    pub fn export_batch_to_rebuildable_root<I>(&self, items: I) -> Result<(), StoreError>
    where
        I: IntoIterator<Item = (IntegrityHash, PathBuf)>,
    {
        self.export_batch_to_mode(items, false)
    }

    fn export_batch_to_mode<I>(
        &self,
        items: I,
        sync_destination_directories: bool,
    ) -> Result<(), StoreError>
    where
        I: IntoIterator<Item = (IntegrityHash, PathBuf)>,
    {
        let items: Vec<_> = items.into_iter().collect();
        let sync_dirs: std::collections::BTreeSet<PathBuf> = if sync_destination_directories {
            items
                .iter()
                .filter(|(_, dest)| !dest.exists())
                .filter_map(|(_, dest)| dest.parent().map(Path::to_path_buf))
                .collect()
        } else {
            std::collections::BTreeSet::new()
        };

        let exported: Result<(), _> = items
            .into_par_iter()
            .try_for_each(|(hash, dest)| self.export_to_mode(&hash, &dest, false, false));

        let sync_result = if sync_destination_directories {
            sync_parent_directories(sync_dirs)
        } else {
            Ok(())
        };
        sync_result?;
        exported
    }
}
