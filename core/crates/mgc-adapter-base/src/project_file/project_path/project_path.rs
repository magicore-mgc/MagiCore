//! Validate project-relative path components and anchor Unix traversal.
//! Kiểm tra thành phần path tương đối và neo traversal Unix vào project root.

use super::super::error::{error, error_with_source};
use mgc_types::MgError;
use std::path::Path;
#[cfg(not(unix))]
use std::path::PathBuf;

#[cfg(unix)]
pub(in crate::project_file) fn project_parent_dir(
    project_root: &Path,
    relative_path: &Path,
) -> Result<(std::fs::File, std::ffi::OsString), MgError> {
    use rustix::fs::{Mode, OFlags, openat};
    use std::os::fd::AsFd;
    use std::path::Component;

    let components: Vec<_> = relative_path.components().collect();
    if components.is_empty()
        || components
            .iter()
            .any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err(error(format!(
            "refusing non-project-relative manifest path: {}",
            relative_path.display()
        )));
    }
    // Open the caller-selected root once and keep its handle as the authority
    // boundary. A symlink used to select the project root is intentional; all
    // components beneath the opened root are still traversed with NOFOLLOW.
    // (Mở project root một lần; symlink chọn root là chủ ý, các cấp dưới cấm.)
    let mut directory = std::fs::File::open(project_root).map_err(|source| {
        error_with_source(source, |e| {
            format!("open project root {}: {e}", project_root.display())
        })
    })?;
    if !directory
        .metadata()
        .map_err(|source| {
            error_with_source(source, |e| {
                format!("inspect project root {}: {e}", project_root.display())
            })
        })?
        .is_dir()
    {
        return Err(error(format!(
            "project root is not a directory: {}",
            project_root.display()
        )));
    }
    for component in &components[..components.len() - 1] {
        let Component::Normal(name) = component else {
            return Err(error("refusing non-project-relative manifest path"));
        };
        let opened = openat(
            directory.as_fd(),
            *name,
            OFlags::RDONLY | OFlags::DIRECTORY | OFlags::NOFOLLOW | OFlags::CLOEXEC,
            Mode::empty(),
        )
        .map_err(|source| {
            error_with_source(source, |e| {
                format!("open project manifest directory without following links: {e}")
            })
        })?;
        directory = std::fs::File::from(opened);
    }
    let Component::Normal(leaf) = components[components.len() - 1] else {
        return Err(error("refusing non-project-relative manifest path"));
    };
    Ok((directory, leaf.to_os_string()))
}

#[cfg(not(unix))]
pub(super) fn checked_project_path(
    project_root: &Path,
    relative_path: &Path,
) -> Result<PathBuf, MgError> {
    use std::path::Component;

    if relative_path.as_os_str().is_empty()
        || relative_path
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err(error(format!(
            "refusing non-project-relative manifest path: {}",
            relative_path.display()
        )));
    }
    // The caller chooses the project root; canonicalizing that one boundary
    // avoids rejecting system aliases such as macOS /var -> /private/var.
    // Root do caller chọn được canonicalize một lần để chấp nhận alias hệ thống của OS.
    let root = std::fs::canonicalize(project_root).map_err(|source| {
        error_with_source(source, |e| {
            format!("resolve project root {}: {e}", project_root.display())
        })
    })?;
    let mut current = root.clone();
    let components: Vec<_> = relative_path.components().collect();
    for (index, component) in components.iter().enumerate() {
        let Component::Normal(name) = component else {
            return Err(error(format!(
                "refusing non-project-relative manifest path: {}",
                relative_path.display()
            )));
        };
        current.push(name);
        let is_leaf = index + 1 == components.len();
        match std::fs::symlink_metadata(&current) {
            Ok(metadata) => {
                let file_type = metadata.file_type();
                if file_type.is_symlink() {
                    return Err(error(format!(
                        "refusing symlinked project manifest component: {}",
                        current.display()
                    )));
                }
                #[cfg(windows)]
                {
                    use std::os::windows::fs::MetadataExt;
                    const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0000_0400;
                    if metadata.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
                        return Err(error(format!(
                            "refusing reparse-point project manifest component: {}",
                            current.display()
                        )));
                    }
                }
                if !is_leaf && !file_type.is_dir() {
                    return Err(error(format!(
                        "project manifest parent is not a directory: {}",
                        current.display()
                    )));
                }
            }
            Err(e) => {
                return Err(error_with_source(e, |e| {
                    format!(
                        "inspect project manifest component {}: {e}",
                        current.display()
                    )
                }));
            }
        }
    }
    if !current.starts_with(&root) {
        return Err(error(format!(
            "project manifest escaped root: {}",
            relative_path.display()
        )));
    }
    Ok(current)
}

#[cfg(test)]
#[path = "test/project_path.rs"]
mod tests;
