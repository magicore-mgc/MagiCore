//! Resolve manifest identity without changing empty-override fallback behavior.
//! Chọn định danh manifest và giữ nguyên fallback khi override rỗng.

pub(crate) fn name_or(manifest: &str, override_name: &str) -> String {
    if override_name.is_empty() {
        manifest.to_string()
    } else {
        override_name.to_string()
    }
}

pub(crate) fn version_or(manifest: &str, override_version: &str) -> String {
    if override_version.is_empty() {
        manifest.to_string()
    } else {
        override_version.to_string()
    }
}
