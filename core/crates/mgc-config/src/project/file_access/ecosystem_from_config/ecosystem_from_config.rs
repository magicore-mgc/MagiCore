//! Parse the ecosystem field from optional TOML content.
//! Parse field ecosystem từ nội dung TOML tùy chọn.
use super::super::error::Result;

/// Keep the existing anyhow boundary for all current project callers.
/// Giữ biên anyhow hiện tại cho mọi caller của project.
pub(in crate::project::project) fn ecosystem_from_config(
    content: Option<&str>,
) -> anyhow::Result<Option<String>> {
    ecosystem_from_config_typed(content).map_err(anyhow::Error::new)
}

/// Parse only the top-level ecosystem value; absent content/value stays None.
/// Chỉ parse ecosystem cấp cao nhất; content/value thiếu tiếp tục là None.
pub(in crate::project::project::file_access) fn ecosystem_from_config_typed(
    content: Option<&str>,
) -> Result<Option<String>> {
    let Some(content) = content else {
        return Ok(None);
    };
    let value: toml::Value = toml::from_str(content).map_err(anyhow::Error::from)?;
    Ok(value
        .get("ecosystem")
        .and_then(|ecosystem| ecosystem.as_str())
        .map(String::from))
}
