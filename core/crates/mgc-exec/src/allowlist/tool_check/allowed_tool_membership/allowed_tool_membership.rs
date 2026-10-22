use crate::allowlist::ALLOWED_TOOLS;
use anyhow::{Result, bail};

/// Reject normalized executables absent from the reviewed allowlist catalog.
/// Từ chối executable chuẩn hóa không có trong catalog allowlist đã review.
pub(in crate::allowlist::tool_check) fn ensure_tool_is_allowed(
    name: &str,
    normalized: &str,
) -> Result<()> {
    if !ALLOWED_TOOLS.contains(&normalized) {
        bail!(
            "tool '{name}' is not on the allowlist (00-index §5.1) — add it there only after review"
        );
    }
    Ok(())
}
