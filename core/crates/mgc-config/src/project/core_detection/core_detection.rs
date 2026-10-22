//! Core detection coordinator and manifest precedence.
//! Điều phối nhận diện core và thứ tự ưu tiên manifest.
use super::super::ProjectConfig;
use super::super::file_access::ensure_regular_project_config;
use super::signatures::{
    add_shared_cloud_ci_markers, detect_app_core, detect_cargo_cores, detect_dotnet_cores,
    detect_go_core, detect_infrastructure_markers, detect_jvm_cores, detect_node_core,
    detect_python_core,
};
use super::{error::CoreDetectionError, manifest_declarations};
use std::path::Path;

impl ProjectConfig {
    /// Collect distinct cores detected from project manifests.
    /// Nhận diện core từ manifest chuẩn, giữ precedence và fail-closed ambiguity.
    fn detect_signatures(project_root: &Path) -> Result<Vec<String>, anyhow::Error> {
        let mut cores = std::collections::BTreeSet::new();
        let (has_cloud_marker, has_game_marker, has_iot_marker) =
            detect_infrastructure_markers(project_root)?;
        if has_cloud_marker {
            cores.insert("clo".to_string());
        }
        if has_game_marker {
            cores.insert("game".to_string());
        }
        if has_iot_marker {
            cores.insert("iot".to_string());
        }

        if let Some(core) = detect_node_core(project_root, has_cloud_marker)? {
            cores.insert(core);
        }
        cores.extend(detect_cargo_cores(
            project_root,
            has_game_marker,
            has_iot_marker,
        )?);
        if let Some(core) = detect_python_core(project_root)? {
            cores.insert(core);
        }
        if detect_app_core(project_root)? {
            cores.insert("app".to_string());
        }
        if let Some(core) = detect_go_core(project_root)? {
            cores.insert(core);
        }
        cores.extend(detect_jvm_cores(project_root)?);
        if !has_game_marker {
            cores.extend(detect_dotnet_cores(project_root)?);
        }
        add_shared_cloud_ci_markers(project_root, &mut cores)?;

        Ok(cores.into_iter().collect())
    }

    /// Detect ecosystem with T9a priority: marker → mgc.toml → signatures.
    ///
    /// Err = ambiguous: multiple ecosystem manifests point at different cores
    /// and no marker (fail-closed — never guess, RULE §9.3).
    pub fn detect_core(project_root: &Path) -> Result<Option<String>, anyhow::Error> {
        if let Some(marker) = Self::read_core_marker(project_root)? {
            return Ok(Some(marker));
        }
        if ensure_regular_project_config(&project_root.join("mgc.toml"))?.is_some() {
            return super::super::file_access::read_to_string_ecosystem(project_root);
        }
        if let Some(core) = manifest_declarations::detect_manifest_core(project_root)
            .map_err(CoreDetectionError::into_anyhow)?
        {
            return Ok(Some(core));
        }
        let signatures = Self::detect_signatures(project_root)?;
        match signatures.len() {
            0 => Ok(None),
            1 => Ok(Some(signatures[0].clone())),
            _ => Err(CoreDetectionError::AmbiguousProjectCore {
                project_root: project_root.to_path_buf(),
                signatures,
                marker_file: Self::CORE_MARKER_FILE.to_string(),
            }
            .into_anyhow()),
        }
    }

    /// Detect the project core without hiding malformed or conflicting identity errors.
    /// Phát hiện core nhưng không biến marker hỏng/xung đột thành kết quả không tìm thấy.
    pub fn auto_detect(project_root: &Path) -> Result<Option<String>, anyhow::Error> {
        Self::detect_core(project_root)
    }
}
