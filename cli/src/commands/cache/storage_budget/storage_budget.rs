//! Measure combined project and MagiCore storage.
//! Đo tổng dung lượng project và dữ liệu MagiCore.

use super::super::status::human_bytes;
use crate::commands::cache::filesystem::canonical_or_original;
use anyhow::Result;
use std::path::Path;
use walkdir::WalkDir;

const DEFAULT_STORAGE_BUDGET_GIB: u64 = 50;
const STORAGE_BUDGET_GIB_ENV: &str = "MGC_STORAGE_BUDGET_GB";
pub(in crate::commands::cache) fn print_storage_budget() -> Result<()> {
    let data_root = mgc_platform::paths::mgc_data_dir();
    let cwd = std::env::current_dir()?;
    let project_root = crate::commands::core::shared::find_project_root(&cwd)?;
    let data_bytes = measured_path_size(&data_root)?;
    let project_bytes = match project_root.as_deref() {
        Some(project) => measured_path_size(project)?,
        None => 0,
    };
    let combined_bytes = project_root
        .as_deref()
        .map(|project| combined_path_size(&data_root, project, data_bytes, project_bytes))
        .unwrap_or(data_bytes);
    let budget_gib = storage_budget_gib();
    let budget_bytes = budget_gib.saturating_mul(1024_u64.pow(3));

    println!(
        "storage:data\t{}\t{}",
        human_bytes(data_bytes),
        data_root.display()
    );
    if let Some(project) = project_root {
        println!(
            "storage:project\t{}\t{}",
            human_bytes(project_bytes),
            project.display()
        );
        println!("storage:combined\t{}", human_bytes(combined_bytes));
    } else {
        println!("storage:project\tproject root not detected");
        println!("storage:combined\t{}", human_bytes(combined_bytes));
    }
    println!("storage:budget\t{} GiB", budget_gib);
    if combined_bytes >= budget_bytes {
        println!(
            "warning\tcombined project and MagiCore data usage has reached the configured budget"
        );
    }
    Ok(())
}

pub(in crate::commands::cache) fn storage_budget_gib() -> u64 {
    std::env::var(STORAGE_BUDGET_GIB_ENV)
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(DEFAULT_STORAGE_BUDGET_GIB)
}

pub(in crate::commands::cache) fn combined_path_size(
    data_root: &Path,
    project_root: &Path,
    data_bytes: u64,
    project_bytes: u64,
) -> u64 {
    let data = canonical_or_original(data_root);
    let project = canonical_or_original(project_root);
    if data.starts_with(&project) {
        project_bytes
    } else if project.starts_with(&data) {
        data_bytes
    } else {
        data_bytes.saturating_add(project_bytes)
    }
}

pub(in crate::commands::cache) fn measured_path_size(path: &Path) -> Result<u64> {
    let metadata = match std::fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(0),
        Err(error) => return Err(error.into()),
    };
    if metadata.is_file() || metadata.file_type().is_symlink() {
        return Ok(metadata.len());
    }

    let mut total = 0u64;
    for entry in WalkDir::new(path).follow_links(false) {
        let entry = entry?;
        let metadata = std::fs::symlink_metadata(entry.path())?;
        if metadata.is_file() || metadata.file_type().is_symlink() {
            total = total.saturating_add(metadata.len());
        }
    }
    Ok(total)
}
