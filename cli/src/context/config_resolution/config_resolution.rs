//! Coordinate persisted identity, config, signature and explicit-core resolution.
//! Điều phối phân giải identity đã lưu, config, chữ ký project và `--core`.

use std::path::{Path, PathBuf};

use mgc_config::project::ProjectConfig;

use super::{configured_project, core_identity, detected_project, unclaimed_project};

/// Resolve one project without changing the established identity precedence.
/// Phân giải một project, giữ nguyên thứ tự ưu tiên identity hiện hành.
pub(in crate::context) fn resolve_config(
    cwd: &Path,
    project_root: Option<&PathBuf>,
    core_override: Option<&str>,
) -> anyhow::Result<(PathBuf, ProjectConfig)> {
    if let Some(root) = project_root {
        // The persisted marker is an ownership record and must be checked first.
        // Marker đã lưu là bản ghi chủ sở hữu nên phải được kiểm tra trước.
        let marker_core = ProjectConfig::read_core_marker(root)?;

        // An explicit selector can confirm identity, but cannot reassign it.
        // `--core` chỉ xác nhận identity, không được đổi chủ sở hữu project.
        if let (Some(requested), Some(owned)) = (core_override, marker_core.as_deref()) {
            core_identity::ensure_override_matches_identity(requested, owned)?;
        }

        if let Some(config) =
            configured_project::resolve(root, marker_core.as_deref(), core_override)?
        {
            return Ok((root.clone(), config));
        }

        if let Some(config) =
            detected_project::resolve(root, marker_core.as_deref(), core_override)?
        {
            return Ok((root.clone(), config));
        }

        if let Some(core) = core_override {
            return Ok((
                root.clone(),
                unclaimed_project::for_project_root(root, core),
            ));
        }

        return Err(crate::error::cannot_detect_project_type(root));
    }

    if let Some(core) = core_override {
        return Ok((
            cwd.to_path_buf(),
            unclaimed_project::without_project_root(core),
        ));
    }

    Err(crate::error::no_mgc_project_root())
}

#[cfg(test)]
#[path = "test/config_resolution.rs"]
mod tests;
