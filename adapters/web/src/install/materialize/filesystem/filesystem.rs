//! Filesystem operations for Web dependency materialization.
//! Thao tác filesystem thuộc materialize dependency Web.

use std::path::Path;

use mgc_types::{MgError, MgResult, PackageId};

pub fn remove_fs_entry(path: &Path) -> MgResult<()> {
    let meta = match std::fs::symlink_metadata(path) {
        Ok(meta) => meta,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(err) => {
            return Err(MgError::Other(format!(
                "failed to inspect filesystem entry '{}': {}",
                path.display(),
                err
            )));
        }
    };

    if meta.file_type().is_dir() && !meta.file_type().is_symlink() {
        std::fs::remove_dir_all(path).map_err(|err| {
            MgError::Other(format!(
                "failed to remove directory '{}': {}",
                path.display(),
                err
            ))
        })?;
    } else {
        std::fs::remove_file(path).map_err(|err| {
            MgError::Other(format!(
                "failed to remove file '{}': {}",
                path.display(),
                err
            ))
        })?;
    }

    Ok(())
}

pub fn reset_nested_node_modules(package_dir: &Path) -> MgResult<()> {
    let nested_node_modules = package_dir.join("node_modules");
    if nested_node_modules.exists() {
        remove_fs_entry(&nested_node_modules)?;
    }
    std::fs::create_dir_all(&nested_node_modules).map_err(|err| {
        MgError::Other(format!(
            "failed to recreate nested node_modules '{}': {}",
            nested_node_modules.display(),
            err
        ))
    })?;
    Ok(())
}

pub fn prune_root_install_dirs(
    node_modules: &Path,
    expected_root_packages: &std::collections::HashMap<String, PackageId>,
) -> MgResult<()> {
    if !node_modules.exists() {
        return Ok(());
    }

    for entry in std::fs::read_dir(node_modules).map_err(|err| {
        MgError::Other(format!(
            "failed to read node_modules '{}': {}",
            node_modules.display(),
            err
        ))
    })? {
        let entry = entry.map_err(|err| {
            MgError::Other(format!(
                "failed to iterate node_modules '{}': {}",
                node_modules.display(),
                err
            ))
        })?;
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        if name == ".bin" || name == ".magicore" {
            continue;
        }

        if name.starts_with('@') && path.is_dir() {
            for scoped_entry in std::fs::read_dir(&path).map_err(|err| {
                MgError::Other(format!(
                    "failed to read scoped directory '{}': {}",
                    path.display(),
                    err
                ))
            })? {
                let scoped_entry = scoped_entry.map_err(|err| {
                    MgError::Other(format!(
                        "failed to iterate scoped directory '{}': {}",
                        path.display(),
                        err
                    ))
                })?;
                let scoped_name = scoped_entry.file_name().to_string_lossy().to_string();
                let package_name = format!("{}/{}", name, scoped_name);
                if !expected_root_packages.contains_key(&package_name) {
                    remove_fs_entry(&scoped_entry.path())?;
                }
            }

            let mut remaining = std::fs::read_dir(&path).map_err(|err| {
                MgError::Other(format!(
                    "failed to re-read scoped directory '{}': {}",
                    path.display(),
                    err
                ))
            })?;
            if remaining.next().is_none() {
                remove_fs_entry(&path)?;
            }
            continue;
        }

        if !expected_root_packages.contains_key(&name) {
            remove_fs_entry(&path)?;
        }
    }

    Ok(())
}

#[cfg(test)]
#[path = "test/filesystem.rs"]
mod tests;
