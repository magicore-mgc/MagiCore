//! Reads a cached package only after containment, file-kind and integrity checks.
//! Chỉ đọc package cache sau khi kiểm tra containment, loại file và integrity.

use crate::package_cache::{
    error::CacheError,
    package_cache::PackageCache,
    path_guard::{ensure_parent_within_root, is_regular_cache_file},
};
use anyhow::Result;
use std::path::PathBuf;

impl PackageCache {
    /// Retrieve a cached package after verifying its expected integrity.
    /// Lấy package trong cache sau khi xác minh integrity mong đợi.
    pub fn get_package(&self, package_id: &str, expected_integrity: &str) -> Result<PathBuf> {
        let _guard = self.locks.read_for(package_id)?;
        let package_path = self.find_cached_path(package_id)?;
        let actual_integrity = self.compute_integrity_no_follow(&package_path)?;
        if actual_integrity != expected_integrity {
            return Err(CacheError::IntegrityMismatch {
                package_id: package_id.to_owned(),
                expected: expected_integrity.to_owned(),
                actual: actual_integrity,
                during_store: false,
            }
            .into());
        }
        Ok(package_path)
    }

    /// Check either the v2 entry or a safe legacy-format entry.
    /// Kiểm tra entry v2 hoặc entry định dạng cũ an toàn.
    pub fn has_package(&self, package_id: &str) -> bool {
        let Ok(_guard) = self.locks.read_for(package_id) else {
            return false;
        };
        let current = self.package_path(package_id);
        match is_regular_cache_file(&self.root, &current) {
            Ok(true) => return true,
            Ok(false) => {}
            Err(_) => return false,
        }
        self.legacy_package_path(package_id)
            .map(|path| is_regular_cache_file(&self.root, &path).unwrap_or(false))
            .unwrap_or(false)
    }

    fn find_cached_path(&self, package_id: &str) -> Result<PathBuf> {
        let current = self.package_path(package_id);
        if is_regular_cache_file(&self.root, &current)? {
            return Ok(current);
        }
        if let Some(legacy) = self.legacy_package_path(package_id)
            && is_regular_cache_file(&self.root, &legacy)?
        {
            ensure_parent_within_root(&self.root, &legacy)?;
            return Ok(legacy);
        }
        Err(CacheError::PackageNotInCache(package_id.to_owned()).into())
    }
}
