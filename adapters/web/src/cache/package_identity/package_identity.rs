//! Derive deterministic paths and keys for extracted package content.
//! Tạo path và key xác định cho nội dung package đã giải nén.

use crate::cache::SharedWebCache;
use mgc_types::adapter::ResolvedPackage;
use std::path::{Path, PathBuf};

impl SharedWebCache {
    pub fn extracted_package_root(&self, pkg: &ResolvedPackage) -> PathBuf {
        crate::cache::shared_extracted_package_root(&self.root, pkg)
    }
}

pub fn shared_extracted_package_root(root: &Path, pkg: &ResolvedPackage) -> PathBuf {
    let safe_name = pkg.id.name_str().replace('/', "__").replace('@', "");
    let cache_key = extracted_package_cache_key(pkg);
    root.join("packages")
        .join(safe_name)
        .join(cache_key)
        .join("package")
}

pub fn extracted_package_cache_key(pkg: &ResolvedPackage) -> String {
    if pkg.integrity.is_empty() {
        return pkg.id.version().to_string();
    }
    let fs_safe = pkg
        .integrity
        .replace('/', "_")
        .replace('+', "-")
        .replace('=', "");
    format!("{}-{}", pkg.id.version(), fs_safe)
}
