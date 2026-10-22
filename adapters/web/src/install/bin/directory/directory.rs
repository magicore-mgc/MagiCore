//! Prepare `.bin` and remove only stale MGC-owned links in strict mode.
//! Chuẩn bị `.bin` và chỉ xóa link MGC cũ trong chế độ strict.

use mgc_types::MgError;
use mgc_types::adapter::ResolvedPackage;
use std::path::{Path, PathBuf};

use super::super::error::BinError;

pub(in crate::install::bin) fn prepare_bin_directory(
    node_modules: &Path,
    expected_root: &[&ResolvedPackage],
    strict: bool,
) -> Result<PathBuf, BinError> {
    let bin_dir = node_modules.join(".bin");
    if !strict {
        if bin_dir.exists() {
            std::fs::remove_dir_all(&bin_dir).map_err(|err| {
                BinError::Directory(MgError::Other(format!(
                    "failed to remove stale bin dir '{}': {}",
                    bin_dir.display(),
                    err
                )))
            })?;
        }
        std::fs::create_dir_all(&bin_dir).map_err(|err| {
            BinError::Directory(MgError::Other(format!(
                "failed to create bin dir '{}': {}",
                bin_dir.display(),
                err
            )))
        })?;
    } else {
        std::fs::create_dir_all(&bin_dir).map_err(|err| {
            BinError::Directory(MgError::Other(format!(
                "failed to create bin dir '{}': {}",
                bin_dir.display(),
                err
            )))
        })?;
        let vstore_keys: std::collections::HashSet<String> = expected_root
            .iter()
            .map(|pkg| {
                format!(
                    "{}@{}",
                    pkg.id.name().as_str().replace('/', "+"),
                    pkg.id.version()
                )
            })
            .collect();
        for entry in std::fs::read_dir(&bin_dir).map_err(|err| {
            BinError::Directory(MgError::Other(format!(
                "failed to read bin dir '{}': {}",
                bin_dir.display(),
                err
            )))
        })? {
            let entry = entry.map_err(|err| {
                BinError::Directory(MgError::Other(format!(
                    "failed to iterate bin dir '{}': {}",
                    bin_dir.display(),
                    err
                )))
            })?;
            let link = entry.path();
            let Ok(target) = std::fs::read_link(&link) else {
                continue;
            };
            let resolved = target.canonicalize();
            let Ok(resolved) = resolved else {
                std::fs::remove_file(&link).map_err(|err| {
                    BinError::Directory(MgError::Other(format!(
                        "failed to remove dangling bin link '{}': {}",
                        link.display(),
                        err
                    )))
                })?;
                continue;
            };
            let resolved_str = resolved.to_string_lossy();
            if !resolved_str.contains(".magicore/") {
                continue;
            }
            let still_expected = vstore_keys
                .iter()
                .any(|key| resolved_str.contains(&format!(".magicore/{key}/node_modules/")));
            if !still_expected {
                std::fs::remove_file(&link).map_err(|err| {
                    BinError::Directory(MgError::Other(format!(
                        "failed to remove stale bin link '{}': {}",
                        link.display(),
                        err
                    )))
                })?;
            }
        }
    }

    Ok(bin_dir)
}
