//! Build a safe target path for local and scoped Web package links.
//! Tạo target path an toàn cho link package Web cục bộ và scoped.

use std::path::{Path, PathBuf};

use super::super::error::WebLinkError;
use super::PackageLinkName;

pub(in crate::commands::core::web::links) fn link_name(
    package: &str,
) -> std::result::Result<PackageLinkName, WebLinkError> {
    if package.starts_with('@') {
        return scoped_package_name(package);
    }

    let path = Path::new(package);
    let is_explicit_path =
        package.starts_with('.') || path.is_absolute() || package.starts_with('/');
    let target = if is_explicit_path {
        path.file_name().and_then(|name| name.to_str())
    } else if package.contains('/') {
        if !package.split('/').all(safe_segment) {
            return Err(WebLinkError::invalid_link_name(package));
        }
        package.rsplit('/').next()
    } else {
        Some(package)
    }
    .filter(|name| safe_segment(name))
    .ok_or_else(|| WebLinkError::invalid_link_name(package))?;

    Ok(PackageLinkName::new(
        PathBuf::from(target),
        target.to_owned(),
    ))
}

fn scoped_package_name(package: &str) -> std::result::Result<PackageLinkName, WebLinkError> {
    let mut segments = package.split('/');
    let scope = segments.next().unwrap_or_default();
    let name = segments.next();
    let valid = scope.len() > 1
        && scope.starts_with('@')
        && safe_segment(&scope[1..])
        && name.is_some_and(safe_segment)
        && segments.next().is_none();
    if !valid {
        return Err(WebLinkError::invalid_link_name(package));
    }
    let name = name.expect("validated scoped package name");

    Ok(PackageLinkName::new(
        PathBuf::from(scope).join(name),
        package.to_owned(),
    ))
}

fn safe_segment(segment: &str) -> bool {
    !segment.is_empty()
        && segment != "."
        && segment != ".."
        && !segment.contains('/')
        && !segment.contains('\\')
        && !segment.ends_with('.')
        && !segment.ends_with(' ')
        && !segment.chars().any(|character| {
            character.is_control() || matches!(character, '<' | '>' | ':' | '"' | '|' | '?' | '*')
        })
}
