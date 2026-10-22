//! Load hook files in path order and merge commands by event.
//! Đọc file hook theo thứ tự path và gộp command theo event.

use anyhow::Result;
use std::path::Path;

use super::super::config::{HooksConfig, load_hook_config};
use super::super::paths::hooks_paths;

/// Merge paths in project-then-user order; duplicate events append commands.
/// Gộp theo thứ tự dự án rồi user; event trùng sẽ nối thêm command.
pub(in crate::hooks::owners) fn load_merged(project_root: &Path) -> Result<HooksConfig> {
    let mut merged = HooksConfig::default();
    for path in hooks_paths(project_root) {
        if let Some(config) = load_hook_config(&path)? {
            merge_config(&mut merged, config);
        }
    }
    Ok(merged)
}

pub(in crate::hooks::owners) fn merge_config(merged: &mut HooksConfig, config: HooksConfig) {
    for (event, commands) in config.hooks {
        merged.hooks.entry(event).or_default().extend(commands);
    }
}
