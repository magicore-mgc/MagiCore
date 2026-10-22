//! Own filesystem age, path, and empty-directory primitives for pruning.
//! Quản lý helper filesystem về tuổi, path và thư mục rỗng cho prune.

use std::path::{Path, PathBuf};

use walkdir::WalkDir;

pub fn cleanup_empty_dirs(root: &Path) {
    if !root.exists() {
        return;
    }
    let mut directories: Vec<PathBuf> = WalkDir::new(root)
        .min_depth(1)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_dir())
        .map(|entry| entry.path().to_path_buf())
        .collect();
    directories.sort_by_key(|path| std::cmp::Reverse(path.components().count()));
    for dir in directories {
        remove_dir_if_empty(&dir);
    }
}

pub fn path_is_older_than(path: &Path, max_age: std::time::Duration) -> bool {
    std::fs::metadata(path)
        .and_then(|metadata| metadata.modified())
        .ok()
        .and_then(|modified| modified.elapsed().ok())
        .is_some_and(|elapsed| elapsed >= max_age)
}

pub fn remove_dir_if_empty(path: &Path) {
    let _ = std::fs::remove_dir(path);
}

pub fn path_to_cache_ref_string(path: &Path) -> String {
    canonical_or_original(path).to_string_lossy().into_owned()
}

pub fn canonical_or_original(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}
