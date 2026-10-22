//! Resolve project files without exposing hidden or out-of-root targets.
//! Resolve file project, không lộ hidden hoặc target ngoài root.

use std::path::{Component, Path, PathBuf};

use super::super::request_path::request_path_is_safe;
use super::SourceFileResolutionError;

#[derive(Clone, Copy)]
enum SearchScope {
    ProjectOnly,
    ProjectAndPublic,
}

/// Resolve a request-relative file to a canonical file inside the project.
/// Resolve file theo request về canonical file nằm trong project.
pub(in crate::bundler::dev_server::implementation) async fn resolve_source_file(
    project_root: &Path,
    rel_path: &str,
) -> Result<Option<PathBuf>, SourceFileResolutionError> {
    resolve_file(project_root, rel_path, SearchScope::ProjectAndPublic).await
}

/// Resolve a fixed project-root document without the public/ fallback.
/// Resolve tài liệu cố định trong project root, không fallback sang public/.
pub(in crate::bundler::dev_server::implementation) async fn resolve_project_file(
    project_root: &Path,
    rel_path: &str,
) -> Result<Option<PathBuf>, SourceFileResolutionError> {
    resolve_file(project_root, rel_path, SearchScope::ProjectOnly).await
}

async fn resolve_file(
    project_root: &Path,
    rel_path: &str,
    search_scope: SearchScope,
) -> Result<Option<PathBuf>, SourceFileResolutionError> {
    if !request_path_is_safe(rel_path) {
        return Err(SourceFileResolutionError::Forbidden);
    }

    let requested = Path::new(rel_path);
    let public_well_known = matches!(search_scope, SearchScope::ProjectAndPublic)
        && is_public_well_known_request(requested);
    if has_private_component(requested, public_well_known, false) {
        return Err(SourceFileResolutionError::Forbidden);
    }

    let canonical_root = tokio::fs::canonicalize(project_root)
        .await
        .map_err(SourceFileResolutionError::Io)?;
    let mut candidates = Vec::with_capacity(2);
    if public_well_known {
        candidates.push(canonical_root.join("public").join(requested));
    } else {
        candidates.push(canonical_root.join(requested));
        if matches!(search_scope, SearchScope::ProjectAndPublic) {
            candidates.push(canonical_root.join("public").join(requested));
        }
    }

    for candidate in candidates {
        let canonical_file = match tokio::fs::canonicalize(&candidate).await {
            Ok(path) => path,
            Err(error)
                if matches!(
                    error.kind(),
                    std::io::ErrorKind::NotFound | std::io::ErrorKind::NotADirectory
                ) =>
            {
                continue;
            }
            Err(error) => return Err(SourceFileResolutionError::Io(error)),
        };

        if !canonical_file.starts_with(&canonical_root) {
            return Err(SourceFileResolutionError::Forbidden);
        }

        let canonical_relative = canonical_file
            .strip_prefix(&canonical_root)
            .map_err(|_| SourceFileResolutionError::Forbidden)?;
        if public_well_known && !is_public_well_known_target(canonical_relative) {
            return Err(SourceFileResolutionError::Forbidden);
        }
        if has_private_component(canonical_relative, public_well_known, true) {
            return Err(SourceFileResolutionError::Forbidden);
        }

        let requested_is_symlink = match tokio::fs::symlink_metadata(&candidate).await {
            Ok(metadata) => metadata.file_type().is_symlink(),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => continue,
            Err(error) => return Err(SourceFileResolutionError::Io(error)),
        };
        if requested_is_symlink && requested.extension() != canonical_file.extension() {
            return Err(SourceFileResolutionError::Forbidden);
        }

        let metadata = tokio::fs::metadata(&canonical_file)
            .await
            .map_err(SourceFileResolutionError::Io)?;
        if metadata.is_file() {
            return Ok(Some(canonical_file));
        }
    }

    Ok(None)
}

fn is_public_well_known_request(path: &Path) -> bool {
    matches!(
        path.components().next(),
        Some(Component::Normal(component)) if component == ".well-known"
    )
}

fn is_public_well_known_target(path: &Path) -> bool {
    let mut components = path.components();
    matches!(
        (components.next(), components.next()),
        (Some(Component::Normal(public)), Some(Component::Normal(well_known)))
            if public == "public" && well_known == ".well-known"
    )
}

fn has_private_component(path: &Path, allow_well_known: bool, project_relative: bool) -> bool {
    let mut normal_component_index = 0;
    let mut first_component_is_public = false;

    for component in path.components() {
        let Component::Normal(value) = component else {
            continue;
        };
        let value = value.to_string_lossy();

        if value.starts_with('.') {
            if !allow_well_known || value != ".well-known" {
                return true;
            }

            let well_known_is_public = if project_relative {
                normal_component_index == 1 && first_component_is_public
            } else {
                normal_component_index == 0
            };
            if !well_known_is_public {
                return true;
            }
        }

        if normal_component_index == 0 {
            first_component_is_public = value == "public";
        }
        normal_component_index += 1;
    }

    false
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
