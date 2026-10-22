use anyhow::{Result, bail};

/// Trim and reject an empty executable name before policy checks.
/// Cắt khoảng trắng và chặn tên executable rỗng trước khi xét policy.
pub(in crate::allowlist::tool_check) fn validate_non_empty_tool_name(name: &str) -> Result<&str> {
    let name = name.trim();
    if name.is_empty() {
        bail!("tool name is empty");
    }
    Ok(name)
}
