//! Resolves the core owning the imported project lock.
//! Xác định core sở hữu lock import của project.

use super::super::ImportError;
use std::path::Path;

/// Select the owner from the authenticated marker, then config, then legacy Web default.
/// Chọn owner theo marker, tiếp đến config, cuối cùng mới dùng mặc định Web cũ.
pub(in crate::commands::import) fn project_lock_owner(root: &Path) -> Result<String, ImportError> {
    let marker = mgc_config::project::ProjectConfig::read_core_marker(root)
        .map_err(ImportError::ProjectConfig)?;
    let core = match marker {
        Some(core) => core,
        None => match mgc_config::project::ProjectConfig::load(root)
            .map_err(ImportError::ProjectConfig)?
        {
            Some(config) => normalize_config_owner(&config.ecosystem),
            None => "web".to_string(),
        },
    };
    validate_core_owner(core)
}

/// Normalize only the legacy config alias used by the previous import path.
/// Chuẩn hóa đúng alias config cũ mà import đang hỗ trợ.
fn normalize_config_owner(ecosystem: &str) -> String {
    let core = ecosystem.trim().to_ascii_lowercase();
    if core == "cloud" {
        "clo".to_string()
    } else {
        core
    }
}

/// Reject unsupported owners before assigning imported packages to a core.
/// Từ chối owner không hỗ trợ trước khi gán package import vào core.
fn validate_core_owner(core: String) -> Result<String, ImportError> {
    if !mgc_config::project::ProjectConfig::KNOWN_CORES.contains(&core.as_str()) {
        return Err(ImportError::UnsupportedCoreOwner { core });
    }
    Ok(core)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
