//! Validate execution arguments against a project boundary.
//! Xác thực đối số thực thi theo ranh giới dự án.

#[path = "error.rs"]
mod error;
use anyhow::{Context, Result};
use error::ArgumentValidationError;
use std::path::{Component, Path, PathBuf};

/// Reject path arguments that cannot be proven to remain within the project.
/// Từ chối đường dẫn không chứng minh được rằng vẫn nằm trong dự án.
pub(in crate::run::implementation) fn validate_args_no_traversal(
    args: &[String],
    project_root: Option<&Path>,
) -> Result<()> {
    let Some(root) = project_root else {
        for arg in args {
            if has_parent_path_component(arg) || is_absolute_without_root(arg) {
                return Err(ArgumentValidationError::MissingProjectRoot {
                    argument: arg.clone(),
                }
                .into());
            }
        }
        return Ok(());
    };

    let canonical_root = root
        .canonicalize()
        .with_context(|| format!("Failed to canonicalize project root: {root:?}"))?;
    // A relative cwd must be resolved against the canonical root before the
    // joiner below, otherwise every relative argument becomes empty-prefixed.
    // (cwd tương đối phải được resolve theo root đã canonicalize trước bước nối
    // bên dưới, nếu không mọi argument tương đối bị ghép vào "" .)
    let canonical_root = if canonical_root.as_os_str().is_empty() {
        std::env::current_dir()?
    } else {
        canonical_root
    };

    for arg in args {
        let looks_like_path = arg.contains('/')
            || arg.contains('\\')
            || arg.starts_with('.')
            || has_windows_drive_prefix(arg);
        if !looks_like_path {
            continue;
        }

        let resolved = if is_rooted_for_current_platform(arg) {
            PathBuf::from(arg)
        } else {
            canonical_root.join(arg)
        };

        match resolved.canonicalize() {
            Ok(canonical_arg) => {
                if !canonical_arg.starts_with(&canonical_root) {
                    return Err(ArgumentValidationError::PathOutsideProject {
                        argument: arg.clone(),
                        resolved: canonical_arg,
                        root: canonical_root.clone(),
                    }
                    .into());
                }
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                let resolved = resolve_missing_path(&resolved)?;
                if !resolved.starts_with(&canonical_root) {
                    return Err(ArgumentValidationError::PathOutsideProject {
                        argument: arg.clone(),
                        resolved,
                        root: canonical_root.clone(),
                    }
                    .into());
                }
            }
            Err(source) => {
                return Err(ArgumentValidationError::CannotResolvePath {
                    path: resolved,
                    source,
                }
                .into());
            }
        }
    }

    Ok(())
}

fn has_parent_path_component(value: &str) -> bool {
    value.split(['/', '\\']).any(|component| component == "..")
}

fn is_absolute_without_root(value: &str) -> bool {
    value.starts_with('/') || value.starts_with('\\') || has_windows_drive_prefix(value)
}

fn has_windows_drive_prefix(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':'
}

#[cfg(windows)]
fn is_rooted_for_current_platform(value: &str) -> bool {
    Path::new(value).is_absolute()
        || value.starts_with('/')
        || value.starts_with('\\')
        || has_windows_drive_prefix(value)
}

#[cfg(not(windows))]
fn is_rooted_for_current_platform(value: &str) -> bool {
    Path::new(value).is_absolute()
}

fn resolve_missing_path(path: &Path) -> Result<PathBuf> {
    let original = path.to_path_buf();
    let mut ancestor = original.clone();
    let mut missing_suffix = Vec::new();

    loop {
        match std::fs::canonicalize(&ancestor) {
            Ok(mut canonical_ancestor) => {
                for component in missing_suffix.iter().rev() {
                    canonical_ancestor.push(component);
                }
                return Ok(normalize_path(&canonical_ancestor));
            }
            Err(source) if source.kind() == std::io::ErrorKind::NotFound => {
                match std::fs::symlink_metadata(&ancestor) {
                    Ok(metadata) if metadata.file_type().is_symlink() => {
                        return Err(ArgumentValidationError::CannotResolvePath {
                            path: ancestor,
                            source,
                        }
                        .into());
                    }
                    Ok(_) => {}
                    Err(metadata_error)
                        if metadata_error.kind() == std::io::ErrorKind::NotFound => {}
                    Err(source) => {
                        return Err(ArgumentValidationError::CannotResolvePath {
                            path: ancestor,
                            source,
                        }
                        .into());
                    }
                }

                let Some(name) = ancestor.file_name().map(|name| name.to_os_string()) else {
                    return Err(ArgumentValidationError::CannotResolvePath {
                        path: ancestor,
                        source,
                    }
                    .into());
                };
                missing_suffix.push(name);
                if !ancestor.pop() {
                    return Err(ArgumentValidationError::CannotResolvePath {
                        path: original,
                        source,
                    }
                    .into());
                }
            }
            Err(source) => {
                return Err(ArgumentValidationError::CannotResolvePath {
                    path: ancestor,
                    source,
                }
                .into());
            }
        }
    }
}

fn normalize_path(path: &Path) -> PathBuf {
    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::Prefix(_) | Component::RootDir | Component::Normal(_) => {
                normalized.push(component.as_os_str());
            }
            Component::CurDir => {}
            Component::ParentDir => {
                if !normalized.pop() && !normalized.has_root() {
                    normalized.push("..");
                }
            }
        }
    }
    normalized
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
