//! Parse explicit MagiCore core declarations from supported manifests.
//! Đọc khai báo core MagiCore tường minh từ các manifest được hỗ trợ.
use std::collections::BTreeSet;
use std::path::Path;

use super::super::error::{CoreDetectionError, Result};
use super::{cargo, package_json, pyproject};

/// Read, canonicalize and validate explicit core declarations.
/// Đọc, chuẩn hóa và kiểm tra các khai báo core tường minh.
pub(in crate::project::project::core_detection) fn detect_manifest_core(
    project_root: &Path,
) -> Result<Option<String>> {
    let mut cores = BTreeSet::new();
    if let Some(core) = package_json::explicit_core(project_root)? {
        cores.insert(core);
    }
    if let Some(core) = cargo::explicit_core(project_root)? {
        cores.insert(core);
    }
    if let Some(core) = pyproject::explicit_core(project_root)? {
        cores.insert(core);
    }

    if cores.len() > 1 {
        return Err(CoreDetectionError::ConflictingManifestDeclarations {
            cores: cores.into_iter().collect(),
            marker_file: crate::project::project::ProjectConfig::CORE_MARKER_FILE.to_string(),
        });
    }
    let Some(core) = cores.into_iter().next() else {
        return Ok(None);
    };
    if !crate::project::project::ProjectConfig::is_known_core(&core) {
        return Err(CoreDetectionError::UnknownDeclaredCore {
            core,
            marker_file: crate::project::project::ProjectConfig::CORE_MARKER_FILE.to_string(),
        });
    }
    Ok(Some(core))
}
