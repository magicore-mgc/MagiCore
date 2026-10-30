//! Core-name normalization and marker/config identity checks.
//! Chuẩn hóa tên core và đối chiếu identity marker/cấu hình.
use std::path::Path;

use super::super::super::ProjectConfig;
use super::super::super::file_access::{
    ensure_regular_project_config, read_regular_project_text, read_to_string_ecosystem,
};
use super::super::error::{CoreMarkerError, Result};

/// Canonicalize a core name (trim, lowercase, alias mapping).
///
/// Delegates to `mgc_types::Ecosystem::canonical_core_name` — the ONE alias
/// table. A local `if name == "cloud"` here was the third compensating layer
/// for the `Ecosystem::as_str()` split (H-1, loop1500).
/// Chuẩn hóa tên core (trim, chữ thường và ánh xạ bí danh) qua bảng alias duy
/// nhất trong `mgc_types`.
pub(in crate::project::project::core_marker) fn canonical_core(name: &str) -> String {
    let name = name.trim().to_ascii_lowercase();
    mgc_types::Ecosystem::canonical_core_name(&name)
}

pub(in crate::project::project::core_marker) fn is_known_core(name: &str) -> bool {
    ProjectConfig::KNOWN_CORES.contains(&name)
}

/// Read and cross-check the plain-text marker without enforcing attestation.
/// Đọc/đối chiếu marker text nhưng chưa enforce attestation.
pub(in crate::project::project::core_marker) fn read_core_marker_identity(
    project_root: &Path,
) -> Result<Option<String>> {
    let path = project_root.join(ProjectConfig::CORE_MARKER_FILE);
    let Some(content) = read_regular_project_text(&path, "core marker")? else {
        return Ok(None);
    };
    let first_line = content
        .lines()
        .next()
        .map(|line| line.split('#').next().unwrap_or("").trim())
        .unwrap_or("");
    let core = canonical_core(first_line);
    if core.is_empty() || !is_known_core(&core) {
        return Err(CoreMarkerError::InvalidMarker {
            path,
            marker: first_line.to_string(),
            known: ProjectConfig::KNOWN_CORES.join(", "),
        });
    }
    validate_marker_matches_project_config(project_root, &core)?;
    Ok(Some(core))
}

/// Reject a marker that disagrees with the persisted project identity.
/// Từ chối marker lệch với identity đã lưu trong cấu hình project.
fn validate_marker_matches_project_config(project_root: &Path, marker: &str) -> Result<()> {
    if ensure_regular_project_config(&project_root.join("mgc.toml"))?.is_none() {
        return Ok(());
    }
    let configured =
        read_to_string_ecosystem(project_root)?.ok_or(CoreMarkerError::MissingConfigEcosystem)?;
    let configured = canonical_core(&configured);
    if !is_known_core(&configured) {
        return Err(CoreMarkerError::UnknownConfiguredCore {
            configured,
            marker: marker.to_string(),
        });
    }
    if configured != marker {
        return Err(CoreMarkerError::MarkerConfigConflict {
            marker: marker.to_string(),
            configured,
        });
    }
    Ok(())
}
