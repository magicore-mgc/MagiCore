//! app manifest recognition.
//! Nhận diện manifest app.
use super::super::shared::signature_file_exists;
use std::path::Path;
pub(in crate::project::project::core_detection) fn detect_app_core(
    project_root: &Path,
) -> anyhow::Result<bool> {
    Ok(signature_file_exists(&project_root.join("pubspec.yaml"))?
        || signature_file_exists(&project_root.join("Package.swift"))?)
}
