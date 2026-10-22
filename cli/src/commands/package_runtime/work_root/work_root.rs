//! Purpose-specific MGC data roots and symlink-resistant directory validation.
//! Root data riêng theo purpose và kiểm tra folder chống symlink.

use anyhow::Result;
use std::path::{Path, PathBuf};

const DLX_WORK_DIRECTORY_COMPONENTS: &[&str] = &["cache", "web", "dlx"];
const CREATE_WORK_DIRECTORY_COMPONENTS: &[&str] = &["cache", "web", "create-initializers"];
const DLX_WORK_PREFIX: &str = "run-";
const CREATE_WORK_PREFIX: &str = "initializer-";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PackageRunPurpose {
    Dlx,
    CreateInitializer,
}

impl PackageRunPurpose {
    pub(in crate::commands::package_runtime) fn label(self) -> &'static str {
        match self {
            Self::Dlx => "dlx",
            Self::CreateInitializer => "create",
        }
    }

    fn work_directory_components(self) -> &'static [&'static str] {
        match self {
            Self::Dlx => DLX_WORK_DIRECTORY_COMPONENTS,
            Self::CreateInitializer => CREATE_WORK_DIRECTORY_COMPONENTS,
        }
    }

    pub(in crate::commands::package_runtime) fn work_prefix(self) -> &'static str {
        match self {
            Self::Dlx => DLX_WORK_PREFIX,
            Self::CreateInitializer => CREATE_WORK_PREFIX,
        }
    }
}
pub(in crate::commands::package_runtime) fn work_root_at(
    data_root: &Path,
    purpose: PackageRunPurpose,
) -> Result<PathBuf> {
    std::fs::create_dir_all(data_root)?;
    let canonical_data_root = data_root.canonicalize()?;
    let mut path = canonical_data_root.clone();
    for component in purpose.work_directory_components() {
        path.push(component);
        match std::fs::symlink_metadata(&path) {
            Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_dir() => {
                return Err(super::registry_package_cache_path_unsafe(&path));
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                match std::fs::create_dir(&path) {
                    Ok(()) => {}
                    Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
                    Err(error) => return Err(error.into()),
                }
            }
            Err(error) => return Err(error.into()),
        }
        let metadata = std::fs::symlink_metadata(&path)?;
        if metadata.file_type().is_symlink() || !metadata.is_dir() || path.canonicalize()? != path {
            return Err(super::registry_package_cache_path_unsafe(&path));
        }
    }
    Ok(path)
}
#[cfg(test)]
#[path = "test/work_root.rs"]
mod tests;
