//! Removes cache entries without following their final symlink.
//! Xóa entry cache mà không đi theo symlink cuối.

use crate::package_cache::{package_cache::PackageCache, path_guard::remove_path_no_follow};
use anyhow::Result;

impl PackageCache {
    /// Invalidate both new and safe legacy entries for a package.
    /// Vô hiệu hóa cả entry mới và entry cũ an toàn của package.
    pub fn invalidate_package(&self, package_id: &str) -> Result<()> {
        let _guard = self.locks.write_for(package_id)?;
        let _ = remove_path_no_follow(&self.root, &self.package_dir(package_id))?;
        if let Some(legacy_dir) = self.legacy_package_dir(package_id) {
            let _ = remove_path_no_follow(&self.root, &legacy_dir)?;
        }
        Ok(())
    }
}
