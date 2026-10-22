//! Compare explicit core selectors with established project identity.
//! So sánh core được chọn với identity project đã xác lập.

use std::path::Path;

use mgc_types::Ecosystem;

use super::super::CoreOverrideConflictError;

/// Reject a core selector that resolves to a different ecosystem.
/// Từ chối lựa chọn core nếu nó chuẩn hóa thành ecosystem khác.
pub(in crate::context::config_resolution) fn ensure_override_matches_identity(
    requested: &str,
    owned: &str,
) -> anyhow::Result<()> {
    let requested_core =
        Ecosystem::from_str(requested).ok_or_else(|| crate::error::unknown_ecosystem(requested))?;
    let owned_core =
        Ecosystem::from_str(owned).ok_or_else(|| crate::error::unknown_ecosystem(owned))?;
    if requested_core != owned_core {
        return Err(
            CoreOverrideConflictError::new(requested_core.as_str(), owned_core.as_str()).into(),
        );
    }
    Ok(())
}

/// Derive a display name from a project path, falling back to `project`.
/// Lấy tên hiển thị từ đường dẫn project, mặc định là `project`.
pub(in crate::context::config_resolution) fn project_name(path: &Path) -> String {
    path.file_name()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_else(|| "project".to_string())
}

#[cfg(test)]
#[path = "test/core_identity.rs"]
mod tests;
