//! Read the authoritative project identity sources in precedence order.
//! Đọc nguồn identity có thẩm quyền theo thứ tự ưu tiên.

use anyhow::Result;
use mgc_config::project::ProjectConfig;
use std::path::Path;

pub(in crate::dispatch) fn read(root: &Path) -> Result<Option<String>> {
    if let Some(core) = ProjectConfig::read_core_marker(root)? {
        return Ok(Some(core));
    }
    if let Some(config) = ProjectConfig::load(root)?
        && !config.ecosystem.is_empty()
    {
        return Ok(Some(config.ecosystem));
    }
    Ok(None)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
