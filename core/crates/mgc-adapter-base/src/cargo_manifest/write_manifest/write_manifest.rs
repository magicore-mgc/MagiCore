//! Read, update, serialize and atomically replace Cargo.toml.
//! Đọc, cập nhật, serialize và thay Cargo.toml theo cách atomic.

use super::super::{
    dependency_table::sync_dependency_table,
    error::{invalid_manifest, parse_error, serialization_error},
};
use mgc_types::Manifest;
use mgc_types::error::MgResult;
use std::path::Path;

pub(in crate::cargo_manifest) fn write_manifest(root: &Path, manifest: &Manifest) -> MgResult<()> {
    let path = root.join("Cargo.toml");
    let content = crate::project_file::read_regular_text(&path, "Cargo.toml")?;
    let mut value: toml::Value = toml::from_str(&content).map_err(parse_error)?;
    let root_table = value
        .as_table_mut()
        .ok_or_else(|| invalid_manifest("Cargo.toml not a table"))?;

    // Update only the two root dependency tables; rebuilding them from the
    // simplified MGC Manifest used to erase Cargo-specific dependency options.
    // (Chỉ cập nhật hai bảng root; dựng lại từng làm mất option riêng của Cargo.)
    sync_dependency_table(
        root_table,
        "dependencies",
        &manifest.dependencies,
        manifest.ecosystem,
    )?;
    sync_dependency_table(
        root_table,
        "dev-dependencies",
        &manifest.dev_dependencies,
        manifest.ecosystem,
    )?;

    let rendered = toml::to_string_pretty(&value).map_err(serialization_error)?;
    crate::project_file::atomic_write_regular(&path, rendered.as_bytes())?;
    Ok(())
}
