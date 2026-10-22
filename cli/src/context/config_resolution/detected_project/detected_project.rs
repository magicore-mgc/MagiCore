//! Detect a project core from signatures and persist its first identity claim.
//! Nhận diện core từ chữ ký và ghi marker ở lần claim identity đầu tiên.

use std::path::Path;

use mgc_config::project::ProjectConfig;

use super::super::{ensure_override_matches_identity, project_name};

/// Return detected project config, or `None` when no signature is recognized.
/// Trả config đã nhận diện, hoặc `None` nếu không tìm thấy chữ ký hỗ trợ.
pub(in crate::context::config_resolution) fn resolve(
    root: &Path,
    marker_core: Option<&str>,
    core_override: Option<&str>,
) -> anyhow::Result<Option<ProjectConfig>> {
    let Some(detected_core) = ProjectConfig::detect_core(root)? else {
        return Ok(None);
    };

    if let Some(requested) = core_override {
        ensure_override_matches_identity(requested, &detected_core)?;
    }
    let selected_core = core_override.unwrap_or(&detected_core);

    if marker_core.is_none() {
        // Persist initial signature detection so later file ordering cannot reassign ownership.
        // Ghi nhận lần nhận diện đầu để thứ tự file về sau không đổi chủ project.
        ProjectConfig::ensure_core_marker_at(root, selected_core)?;
        mgc_ui::warning_stderr(&format!(
            "No '{}' found — claiming project identity as core '{}'. Core reassignment is blocked until an explicit migration workflow is available.",
            ProjectConfig::CORE_MARKER_FILE,
            selected_core,
        ));
    }

    let name = project_name(root);
    Ok(Some(ProjectConfig::new(name, selected_core)))
}

#[cfg(test)]
#[path = "test/detected_project.rs"]
mod tests;
