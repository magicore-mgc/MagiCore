//! Public hook event listing.
//! Liệt kê event hook công khai.

use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;

use super::super::config::HooksConfig;
use super::super::merge::load_merged;

/// List configured hook events.
/// Liệt kê event hook đã cấu hình.
pub fn list_hooks(project_root: &Path) -> Result<HashMap<String, Vec<String>>> {
    let config = load_merged(project_root)?;
    Ok(list_configured_hooks(config))
}

pub(in crate::hooks::owners) fn list_configured_hooks(
    config: HooksConfig,
) -> HashMap<String, Vec<String>> {
    config.hooks
}
