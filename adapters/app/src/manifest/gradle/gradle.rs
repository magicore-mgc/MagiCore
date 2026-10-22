//! Stable public Gradle manifest facade.
//! Facade công khai ổn định cho manifest Gradle.

use mgc_types::{Manifest, MgResult};

use super::CatalogEntry;
use std::path::Path;

use super::error::GradleManifestError;

/// Reject parsing executable Gradle DSL using the existing public error contract.
/// Từ chối parse DSL Gradle thực thi theo hợp đồng lỗi công khai hiện tại.
pub fn parse_build_gradle(project_root: &Path) -> MgResult<Manifest> {
    Err(super::dsl_policy::parse_error(project_root).into_mg_error())
}

/// Reject generic rewrites of executable Gradle build files.
/// Từ chối ghi lại build file Gradle như manifest phụ thuộc chung.
pub fn write_build_gradle(_project_root: &Path, _manifest: &Manifest) -> MgResult<()> {
    Err(super::dsl_policy::write_error().into_mg_error())
}

/// Read resolvable entries from `gradle/libs.versions.toml` when present.
/// Đọc các entry resolve được từ `gradle/libs.versions.toml` nếu có.
pub fn parse_version_catalog(project_root: &Path) -> Option<Vec<CatalogEntry>> {
    super::catalog_reader::parse_version_catalog(project_root)
}

/// Update one version-catalog pin and verify the written value by reading it again.
/// Cập nhật một pin version catalog và xác minh bằng cách đọc lại giá trị đã ghi.
pub fn bump_catalog_pin(
    project_root: &Path,
    group: &str,
    artifact: &str,
    new_version: &str,
) -> MgResult<bool> {
    super::catalog_pin::bump_catalog_pin(project_root, group, artifact, new_version)
        .map_err(GradleManifestError::into_mg_error)
}
