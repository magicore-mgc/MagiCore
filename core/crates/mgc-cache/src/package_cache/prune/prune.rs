//! Coordinates full prune with all package operations.
//! Đồng bộ prune toàn phần với mọi thao tác package.

use crate::package_cache::{
    error::CacheError, package_cache::PackageCache, path_guard::remove_path_no_follow,
};
use anyhow::Result;
use std::fs;
use std::io::ErrorKind;

impl PackageCache {
    /// Remove cached package directories after excluding concurrent readers and writers.
    /// Xóa thư mục package trong cache sau khi chặn reader/writer đồng thời.
    pub fn prune(&self) -> Result<usize> {
        let _guards = self.locks.write_all()?;
        let packages_dir = self.root.join("packages");
        match fs::symlink_metadata(&packages_dir) {
            Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_dir() => {
                return Err(CacheError::UnsafeCachePath(packages_dir).into());
            }
            Ok(_) => {}
            Err(error) if error.kind() == ErrorKind::NotFound => return Ok(0),
            Err(error) => return Err(error.into()),
        }
        crate::package_cache::path_guard::ensure_parent_within_root(
            &self.root,
            &packages_dir.join("sentinel"),
        )?;

        let mut count = 0;
        for entry in fs::read_dir(&packages_dir)? {
            let path = entry?.path();
            let metadata = fs::symlink_metadata(&path)?;
            if (metadata.is_dir() || metadata.file_type().is_symlink())
                && remove_path_no_follow(&self.root, &path)?
            {
                count += 1;
            }
        }
        Ok(count)
    }
}
