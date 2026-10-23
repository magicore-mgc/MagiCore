//! Hardlink-count checks used when pruning cache entries.
//! Kiểm số hardlink dùng khi prune entry cache.

use std::path::Path;

pub fn file_has_no_external_hardlinks(path: &Path) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        std::fs::metadata(path)
            .map(|metadata| metadata.nlink() <= 1)
            .unwrap_or(false)
    }
    #[cfg(not(unix))]
    {
        let _ = path;
        false
    }
}
