//! Enumerate marked package-root directories in the shared Web cache.
//! Liệt kê thư mục package root có marker trong shared Web cache.

use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub(in crate::commands::cache::prune::web_shared) fn web_shared_package_roots(
    root: &Path,
) -> Vec<PathBuf> {
    let packages = root.join("packages");
    if !packages.exists() {
        return Vec::new();
    }

    WalkDir::new(packages)
        .min_depth(1)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_dir())
        .map(|entry| entry.path().to_path_buf())
        .filter(|path| path.join(".magicore-package-root.json").exists())
        .collect()
}
