//! Load configured projects and claim legacy core identity when needed.
//! Nạp project có cấu hình và claim core cho project legacy khi cần.

use std::path::Path;

use mgc_config::project::ProjectConfig;
use mgc_types::Ecosystem;

use super::super::ensure_override_matches_identity;

/// Return configured project data, or `None` when no `mgc.toml` exists.
/// Trả cấu hình project, hoặc `None` khi chưa có `mgc.toml`.
pub(in crate::context::config_resolution) fn resolve(
    root: &Path,
    marker_core: Option<&str>,
    core_override: Option<&str>,
) -> anyhow::Result<Option<ProjectConfig>> {
    let Some(mut config) = ProjectConfig::load(root)? else {
        return Ok(None);
    };

    if let Some(core) = core_override {
        ensure_override_matches_identity(core, &config.ecosystem)?;
        let canonical =
            Ecosystem::from_str(core).ok_or_else(|| crate::error::unknown_ecosystem(core))?;
        config.ecosystem = canonical.as_str().to_string();
    } else if let Some(marker) = marker_core {
        // Marker conflicts were rejected before this owner was called.
        // Marker mâu thuẫn đã bị từ chối trước khi gọi owner này.
        if config.ecosystem != marker {
            config.ecosystem = marker.to_string();
        }
    }

    if marker_core.is_none() {
        // Adopt legacy config identity before any core-aware command can act.
        // Claim identity từ config legacy trước khi lệnh theo core được chạy.
        let canonical = Ecosystem::from_str(&config.ecosystem)
            .ok_or_else(|| crate::error::unknown_ecosystem(&config.ecosystem))?;
        match ProjectConfig::ensure_core_marker_at(root, canonical.as_str()) {
            Ok(()) => mgc_ui::warning(&format!(
                "No '{}' found — recording the existing mgc.toml core '{}' as project identity.",
                ProjectConfig::CORE_MARKER_FILE,
                canonical.as_str(),
            )),
            Err(error)
                if error
                    .downcast_ref::<std::io::Error>()
                    .is_some_and(|io| io.kind() == std::io::ErrorKind::PermissionDenied) =>
            {
                mgc_ui::warning(&format!(
                    "Cannot persist '{}' because the project directory is read-only; using the existing mgc.toml core '{}' as identity.",
                    ProjectConfig::CORE_MARKER_FILE,
                    canonical.as_str(),
                ));
            }
            Err(error) => return Err(error),
        }
    }

    Ok(Some(config))
}

#[cfg(test)]
#[path = "test/configured_project.rs"]
mod tests;
