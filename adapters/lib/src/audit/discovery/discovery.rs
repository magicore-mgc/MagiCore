//! Detect PEP 751 versioned pylock filenames for Python/Rust aggregation.
//! Phát hiện tên pylock PEP 751 có version để chọn aggregation Python/Rust.

use std::path::Path;

/// True when any PEP 751 pylock.*.toml variant exists (mirrors the
/// scanner's discovery rule so the aggregate path triggers identically).
/// Có biến thể pylock.*.toml PEP 751 nào tồn tại (khớp luật discovery
/// của scanner để đường aggregate kích hoạt y hệt).
pub(in crate::audit) fn pylock_variant_exists(project_root: &Path) -> bool {
    std::fs::read_dir(project_root)
        .map(|entries| {
            entries.filter_map(|e| e.ok()).any(|e| {
                e.file_name()
                    .to_str()
                    .is_some_and(|n| n.starts_with("pylock.") && n.ends_with(".toml"))
            })
        })
        .unwrap_or(false)
}
