//! Validate POSIX member paths and enumerate the installed site without links.
//! Kiểm tra path POSIX và duyệt site đã cài mà không theo symlink.

use mgc_types::{MgError, MgResult};
use std::path::{Path, PathBuf};

/// Reject wheel member paths that can escape their extraction root.
/// Từ chối path member wheel có thể thoát khỏi root giải nén.
pub(in crate::protocols::pypi) fn validate_record_path(relative: &str) -> MgResult<()> {
    if relative.is_empty()
        || relative.starts_with('/')
        || relative.starts_with('\\')
        || relative.contains('\\')
        || relative.contains(':')
        || relative.contains('\0')
        || relative
            .split('/')
            .any(|component| component.is_empty() || component == "." || component == "..")
    {
        return Err(MgError::Integrity(format!(
            "unsafe Python wheel member path '{relative}'"
        )));
    }
    Ok(())
}

/// Check each installed path component using no-follow metadata.
/// Kiểm tra từng thành phần path đã cài bằng metadata không theo symlink.
pub(in crate::protocols::pypi) fn verified_site_file_path(
    site: &Path,
    relative: &str,
) -> MgResult<PathBuf> {
    let components: Vec<_> = relative.split('/').collect();
    let mut current = site.to_path_buf();
    for (index, component) in components.iter().enumerate() {
        current.push(component);
        let metadata = std::fs::symlink_metadata(&current).map_err(|error| {
            MgError::Integrity(format!(
                "Python runtime materialization is missing '{relative}': {error}"
            ))
        })?;
        if metadata.file_type().is_symlink() {
            return Err(MgError::Integrity(format!(
                "Python runtime materialization contains a symlink in '{relative}'"
            )));
        }
        let is_last = index + 1 == components.len();
        if (is_last && !metadata.file_type().is_file())
            || (!is_last && !metadata.file_type().is_dir())
        {
            return Err(MgError::Integrity(format!(
                "Python runtime materialization has an invalid path component in '{relative}'"
            )));
        }
    }
    Ok(current)
}

/// Enumerate site files and reject symlinks or special filesystem objects.
/// Liệt kê file site và từ chối symlink hay object filesystem đặc biệt.
pub(in crate::protocols::pypi) fn collect_regular_files(
    root: &Path,
    current: &Path,
    files: &mut std::collections::BTreeSet<String>,
) -> MgResult<()> {
    for entry in std::fs::read_dir(current)? {
        let entry = entry?;
        let path = entry.path();
        let metadata = std::fs::symlink_metadata(&path)?;
        if metadata.file_type().is_symlink() {
            return Err(MgError::Integrity(format!(
                "Python runtime site contains symlink '{}'",
                path.display()
            )));
        }
        if metadata.file_type().is_dir() {
            collect_regular_files(root, &path, files)?;
        } else if metadata.file_type().is_file() {
            let relative = path.strip_prefix(root).map_err(|_| {
                MgError::Integrity("Python runtime site path escaped its root".to_string())
            })?;
            let relative = relative
                .components()
                .map(|component| {
                    component.as_os_str().to_str().ok_or_else(|| {
                        MgError::Integrity(
                            "Python runtime site contains a non-UTF-8 path".to_string(),
                        )
                    })
                })
                .collect::<MgResult<Vec<_>>>()?
                .join("/");
            files.insert(relative);
        } else {
            return Err(MgError::Integrity(format!(
                "Python runtime site contains special file '{}'",
                path.display()
            )));
        }
    }
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
