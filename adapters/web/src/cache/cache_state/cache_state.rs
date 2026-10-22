//! Own shared-cache root discovery, setup, and prune-once lifecycle.
//! Quản lý phát hiện root cache chung, khởi tạo và vòng đời prune một lần.

use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

use mgc_store::{Layout, PackageCache};
use serde::{Deserialize, Serialize};

use crate::cache::read_shared_cache_pinned_package_roots;
use crate::cache_prune::{
    prune_old_files_under, prune_old_metadata_dirs_under, prune_old_package_dirs_under,
    prune_shared_cache_to_quota, shared_cache_max_age_secs, shared_cache_max_bytes,
    shared_cache_prune_due, write_shared_cache_prune_stamp,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedWebCache {
    pub root: PathBuf,
}

impl SharedWebCache {
    pub fn discover() -> Option<Self> {
        // Priority order: MGC_CACHE_DIR → MAGICORE_SHARED_CACHE_DIR → system cache.
        // Thứ tự ưu tiên: MGC_CACHE_DIR → MAGICORE_SHARED_CACHE_DIR → cache hệ thống.
        // MGC_CACHE_DIR is the standard cache override used by tests and CI.
        // MGC_CACHE_DIR là biến chuẩn để ghi đè cache trong test và CI.
        if let Ok(path) = std::env::var("MGC_CACHE_DIR") {
            let trimmed = path.trim();
            if !trimmed.is_empty() {
                let candidate = Self {
                    root: PathBuf::from(trimmed).join("web"),
                };
                if candidate.is_usable() {
                    candidate.maybe_prune_once_per_process();
                    return Some(candidate);
                }
                return None;
            }
        }

        if let Ok(path) = std::env::var("MAGICORE_SHARED_CACHE_DIR") {
            let trimmed = path.trim();
            if !trimmed.is_empty() {
                let candidate = Self {
                    root: PathBuf::from(trimmed),
                };
                if candidate.is_usable() {
                    candidate.maybe_prune_once_per_process();
                    return Some(candidate);
                }
                return None;
            }
        }

        let candidate = Self {
            root: mgc_platform::paths::mgc_data_dir()
                .join("cache")
                .join("web"),
        };
        if candidate.is_usable() {
            candidate.maybe_prune_once_per_process();
            Some(candidate)
        } else {
            None
        }
    }

    pub fn is_usable(&self) -> bool {
        if std::fs::create_dir_all(&self.root).is_err() {
            return false;
        }

        let probe = self.root.join(".mgc-write-probe");
        match std::fs::create_dir(&probe) {
            Ok(()) => {
                let _ = std::fs::remove_dir(&probe);
                true
            }
            Err(_) => false,
        }
    }

    pub fn package_cache(&self) -> anyhow::Result<PackageCache> {
        let layout = Layout::new(self.root.clone());
        std::fs::create_dir_all(layout.root())?;
        PackageCache::new(layout.cache_dir())
    }

    pub fn maybe_prune(&self) {
        if !shared_cache_prune_due(&self.root) {
            return;
        }

        let pinned_roots = read_shared_cache_pinned_package_roots(&self.root);
        let max_age = std::time::Duration::from_secs(shared_cache_max_age_secs());
        let _ = prune_old_files_under(&self.root.join("cache"), max_age);
        let _ = prune_old_files_under(&self.root.join("resolutions"), max_age);
        let _ = prune_old_package_dirs_under(&self.root.join("packages"), max_age, &pinned_roots);
        let _ = prune_old_metadata_dirs_under(&self.root.join("metadata"), max_age);
        let _ = prune_shared_cache_to_quota(&self.root, shared_cache_max_bytes(), &pinned_roots);
        let _ = write_shared_cache_prune_stamp(&self.root);
    }

    pub fn maybe_prune_once_per_process(&self) {
        static PRUNED_ROOTS: OnceLock<Mutex<std::collections::HashSet<PathBuf>>> = OnceLock::new();
        let pruned_roots =
            PRUNED_ROOTS.get_or_init(|| Mutex::new(std::collections::HashSet::new()));
        let mut guard = match pruned_roots.lock() {
            Ok(guard) => guard,
            Err(_) => return,
        };
        if !guard.insert(self.root.clone()) {
            return;
        }
        drop(guard);
        self.maybe_prune();
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
