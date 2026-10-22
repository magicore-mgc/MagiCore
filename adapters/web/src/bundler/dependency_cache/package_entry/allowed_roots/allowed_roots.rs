//! Canonical roots that may contribute files to an installed dependency bundle.
//! Root canonical được phép cung cấp file cho bundle dependency đã cài.

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use super::super::PackageEntryError;

/// Collect package roots and known package-manager stores without allowing
/// arbitrary files placed directly under `node_modules`.
/// Lấy root package và store PM đã biết, không mở quyền cho file tùy ý nằm
/// trực tiếp dưới `node_modules`.
pub(in crate::bundler::dependency_cache) fn allowed_dependency_roots(
    node_modules: &Path,
) -> Result<Vec<PathBuf>, PackageEntryError> {
    let canonical_modules = fs::canonicalize(node_modules).map_err(PackageEntryError::Io)?;
    if !fs::metadata(&canonical_modules)
        .map_err(PackageEntryError::Io)?
        .is_dir()
    {
        return Err(PackageEntryError::ForbiddenPath);
    }

    let mut roots = Vec::new();
    for store in [".pnpm", ".bun", ".store"] {
        add_directory_root(node_modules.join(store), &mut roots)?;
    }
    let store_roots = roots.clone();
    collect_package_roots(&canonical_modules, &store_roots, &mut roots)?;

    roots.sort();
    roots.dedup();
    Ok(roots)
}

fn collect_package_roots(
    node_modules: &Path,
    store_roots: &[PathBuf],
    roots: &mut Vec<PathBuf>,
) -> Result<(), PackageEntryError> {
    let mut pending = vec![node_modules.to_path_buf()];
    let mut visited_module_dirs = HashSet::new();

    while let Some(module_dir) = pending.pop() {
        if !is_directory(&module_dir)? {
            continue;
        }
        let canonical_modules = fs::canonicalize(module_dir).map_err(PackageEntryError::Io)?;
        if !visited_module_dirs.insert(canonical_modules.clone()) {
            continue;
        }

        for item in fs::read_dir(&canonical_modules).map_err(PackageEntryError::Io)? {
            let item = item.map_err(PackageEntryError::Io)?;
            let path = item.path();
            let name = item.file_name();
            let name = name.to_string_lossy();

            if name.starts_with('@') {
                if is_directory(&path)? {
                    for package in fs::read_dir(&path).map_err(PackageEntryError::Io)? {
                        add_package_root(
                            package.map_err(PackageEntryError::Io)?.path(),
                            store_roots,
                            roots,
                            &mut pending,
                        )?;
                    }
                }
            } else if !name.starts_with('.') {
                add_package_root(path, store_roots, roots, &mut pending)?;
            }
        }
    }

    Ok(())
}

fn add_package_root(
    path: PathBuf,
    store_roots: &[PathBuf],
    roots: &mut Vec<PathBuf>,
    pending: &mut Vec<PathBuf>,
) -> Result<(), PackageEntryError> {
    if !is_directory(&path)? {
        return Ok(());
    }
    let canonical_package = fs::canonicalize(path).map_err(PackageEntryError::Io)?;
    if store_roots
        .iter()
        .any(|store| canonical_package.starts_with(store))
    {
        return Ok(());
    }

    roots.push(canonical_package.clone());
    pending.push(canonical_package.join("node_modules"));
    Ok(())
}

fn add_directory_root(path: PathBuf, roots: &mut Vec<PathBuf>) -> Result<(), PackageEntryError> {
    if is_directory(&path)? {
        roots.push(fs::canonicalize(path).map_err(PackageEntryError::Io)?);
    }
    Ok(())
}

fn is_directory(path: &Path) -> Result<bool, PackageEntryError> {
    match fs::metadata(path) {
        Ok(metadata) => Ok(metadata.is_dir()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(PackageEntryError::Io(error)),
    }
}
