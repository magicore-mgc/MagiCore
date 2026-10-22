//! infrastructure manifest recognition.
//! Nhận diện manifest infrastructure.
use super::super::shared::{
    has_regular_extension_file, signature_file_exists, signature_file_exists_in_project,
};
use std::path::Path;
pub(in crate::project::project::core_detection) fn detect_infrastructure_markers(
    project_root: &Path,
) -> anyhow::Result<(bool, bool, bool)> {
    let has_cloud_marker = ["cdk.json", "Pulumi.yaml", "Pulumi.yml", "wrangler.toml"]
        .iter()
        .try_fold(false, |found, name| {
            Ok::<_, anyhow::Error>(found || signature_file_exists(&project_root.join(name))?)
        })?
        || has_regular_extension_file(project_root, "tf")?;
    let has_game_marker = signature_file_exists(&project_root.join("project.godot"))?
        || signature_file_exists_in_project(
            project_root,
            Path::new("ProjectSettings/ProjectVersion.txt"),
        )?
        || has_regular_extension_file(project_root, "uproject")?;
    let has_iot_marker = signature_file_exists(&project_root.join("platformio.ini"))?
        || signature_file_exists(&project_root.join("west.yml"))?;
    Ok((has_cloud_marker, has_game_marker, has_iot_marker))
}

pub(in crate::project::project::core_detection) fn add_shared_cloud_ci_markers(
    project_root: &Path,
    cores: &mut std::collections::BTreeSet<String>,
) -> anyhow::Result<()> {
    // Wrangler and common CI files identify both Cloud and CI/CD; preserve the existing ambiguity guard.
    // Wrangler và file CI chung nhận diện Cloud lẫn CI/CD; giữ nguyên guard mơ hồ hiện có.
    if signature_file_exists(&project_root.join("wrangler.toml"))? {
        cores.insert("cicd".to_string());
        cores.insert("clo".to_string());
    }
    Ok(())
}
