//! Create a filesystem link to a local or globally installed Web package.
//! Tạo filesystem link đến package Web cục bộ hoặc global.

use anyhow::Result;
use mgc_types::adapter::PackageAdapter;
use std::path::Path;

use super::super::{error::WebLinkError, package_input::require_web_package};
use super::super::{package_name::link_name, package_source::find_package_source};

pub async fn link(adapter: &dyn PackageAdapter, root: &Path, package: Option<&str>) -> Result<()> {
    let package =
        require_web_package(adapter.name(), "link", package).map_err(anyhow::Error::new)?;
    mgc_ui::info(&format!("Linking {}...", package));

    let node_modules = root.join("node_modules");
    let name = link_name(package).map_err(anyhow::Error::new)?;
    let link_path = node_modules.join(name.target_path());
    ensure_link_target_available(&link_path, name.display_name()).map_err(anyhow::Error::new)?;

    std::fs::create_dir_all(&node_modules).map_err(WebLinkError::filesystem)?;
    let source = find_package_source(root, package)?;
    ensure_package_parent(&node_modules, &link_path, name.display_name(), true)
        .map_err(anyhow::Error::new)?;
    create_package_link(&source, &link_path).map_err(WebLinkError::filesystem)?;

    mgc_ui::success(&format!(
        "Linked {} -> {}",
        name.display_name(),
        source.display()
    ));
    Ok(())
}

pub(in crate::commands::core::web::links) fn ensure_package_parent(
    node_modules: &Path,
    link_path: &Path,
    display_name: &str,
    create_missing: bool,
) -> std::result::Result<(), WebLinkError> {
    let parent = link_path
        .parent()
        .ok_or_else(|| WebLinkError::unsafe_link_parent(display_name))?;
    if parent == node_modules {
        return Ok(());
    }

    if create_missing {
        match std::fs::create_dir(parent) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(error) => return Err(WebLinkError::filesystem(error)),
        }
    }

    let metadata = match std::fs::symlink_metadata(parent) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound && !create_missing => {
            return Err(WebLinkError::unlink_not_linked(display_name));
        }
        Err(error) => return Err(WebLinkError::filesystem(error)),
    };
    if !metadata.file_type().is_dir() || metadata.file_type().is_symlink() {
        return Err(WebLinkError::unsafe_link_parent(display_name));
    }
    Ok(())
}

pub(super) fn ensure_link_target_available(
    link_path: &Path,
    display_name: &str,
) -> std::result::Result<(), WebLinkError> {
    match std::fs::symlink_metadata(link_path) {
        Ok(_) => Err(WebLinkError::link_exists(display_name)),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(WebLinkError::filesystem(error)),
    }
}

pub(super) fn create_package_link(source: &Path, link_path: &Path) -> std::io::Result<()> {
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(source, link_path)
    }

    #[cfg(windows)]
    {
        if source.is_dir() {
            std::os::windows::fs::symlink_dir(source, link_path)
        } else {
            std::os::windows::fs::symlink_file(source, link_path)
        }
    }
}
