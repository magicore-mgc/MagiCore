//! Ordered parallel wrappers around single-package cache operations.
//! Wrapper song song có thứ tự quanh thao tác cache một package.

use crate::package_cache::package_cache::PackageCache;
use anyhow::Result;
use std::path::PathBuf;

impl PackageCache {
    /// Retrieve multiple packages concurrently while preserving request order.
    /// Lấy nhiều package đồng thời và giữ thứ tự request.
    pub fn get_packages_parallel(&self, requests: &[(String, String)]) -> Result<Vec<PathBuf>> {
        use rayon::prelude::*;

        requests
            .par_iter()
            .map(|(package_id, integrity)| self.get_package(package_id, integrity))
            .collect()
    }

    /// Store multiple packages concurrently while preserving request order.
    /// Lưu nhiều package đồng thời và giữ thứ tự request.
    pub fn store_packages_parallel(
        &self,
        requests: &[(String, PathBuf, String)],
    ) -> Result<Vec<PathBuf>> {
        use rayon::prelude::*;

        requests
            .par_iter()
            .map(|(package_id, source, integrity)| {
                self.store_package(package_id, source, integrity)
            })
            .collect()
    }
}
