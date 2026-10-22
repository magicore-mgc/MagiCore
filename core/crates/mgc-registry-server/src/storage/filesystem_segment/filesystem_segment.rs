//! Rejects unsafe filesystem path segments used by OCI storage.
//! Từ chối segment filesystem không an toàn được dùng bởi OCI storage.

use super::super::prelude::*;

/// Validate a repo/uuid/path segment before it reaches the filesystem.
/// OCI repo names may contain `/` — each component is validated separately.
/// Only [A-Za-z0-9._-] allowed per component, no dots-only (".", ".."),
/// no separators, no backslash.
///
/// Validate segment repo/uuid/path trước khi chạm filesystem. Tên repo OCI
/// có thể chứa `/` — validate từng component: chỉ cho phép [A-Za-z0-9._-],
/// cấm toàn dấu chấm, ký tự phân tách và backslash.
pub(in crate::storage) fn validate_fs_segment(segment: &str) -> Result<()> {
    const MAX_SEGMENT_LEN: usize = 255;
    let bad = |s: &str| {
        s.is_empty()
            || s.len() > MAX_SEGMENT_LEN
            || s == "."
            || s == ".."
            || s.contains('\\')
            || s.contains('/')
            || !s
                .bytes()
                .all(|c| c.is_ascii_alphanumeric() || c == b'.' || c == b'_' || c == b'-')
    };
    // A whole-segment slash is legal only for repos (OCI path names); split
    // and check every component. UUIDs and single segments have no '/'.
    // Dấu `/` chỉ hợp pháp trong repo (tên OCI path) — tách và check từng
    // component. UUID và segment đơn không chứa '/'.
    if segment.contains('/') {
        for component in segment.split('/') {
            if bad(component) {
                anyhow::bail!("invalid filesystem segment: '{segment}' (component '{component}')");
            }
        }
        return Ok(());
    }
    if bad(segment) {
        anyhow::bail!("invalid filesystem segment: '{segment}'");
    }
    Ok(())
}
