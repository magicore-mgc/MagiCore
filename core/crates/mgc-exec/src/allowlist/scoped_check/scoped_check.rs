use crate::allowlist::{ALLOWED_TOOLS, FORBIDDEN_TOOLS, normalize_script_token};
use anyhow::{Result, bail};
use std::path::Path;

/// Scoped check kept for callers that pass cwd, but PM tools stay forbidden everywhere.
/// Kiểm theo cwd vẫn tồn tại để giữ API ổn định; mọi PM ngoài vẫn bị chặn tuyệt đối.
pub fn check_tool_scoped(name: &str, cwd: Option<&Path>) -> Result<()> {
    let _ = cwd;
    let name = name.trim();
    if name.is_empty() {
        bail!("tool name is empty");
    }
    let normalized = normalize_script_token(name).unwrap_or_else(|| name.to_ascii_lowercase());
    if FORBIDDEN_TOOLS.contains(&normalized.as_str()) {
        bail!(
            "tool '{name}' is permanently forbidden (mgc resolver covers its format — use `mgc install` instead)"
        );
    }
    if !ALLOWED_TOOLS.contains(&normalized.as_str()) {
        bail!(
            "tool '{name}' is not on the allowlist (00-index §5.1) — add it there only after review"
        );
    }
    Ok(())
}
