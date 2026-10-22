//! Remove a package entry from the current Web project's `node_modules`.
//! Gỡ entry package khỏi `node_modules` của Web project hiện tại.

use anyhow::Result;
use mgc_types::adapter::PackageAdapter;
use std::path::Path;

use super::super::error::WebLinkError;
use super::super::link::ensure_package_parent;
use super::super::{package_input::require_web_package, package_name::link_name};

pub async fn unlink(
    adapter: &dyn PackageAdapter,
    root: &Path,
    package: Option<&str>,
) -> Result<()> {
    let package =
        require_web_package(adapter.name(), "unlink", package).map_err(anyhow::Error::new)?;
    mgc_ui::info(&format!("Unlinking {}...", package));

    let name = link_name(package).map_err(anyhow::Error::new)?;
    let node_modules = root.join("node_modules");
    let link_path = node_modules.join(name.target_path());
    ensure_package_parent(&node_modules, &link_path, name.display_name(), false)
        .map_err(anyhow::Error::new)?;

    remove_package_entry(&link_path, name.display_name())?;
    mgc_ui::success(&format!("Unlinked {}", package));
    Ok(())
}

pub(super) fn remove_package_entry(
    link_path: &Path,
    package_name: &str,
) -> std::result::Result<(), WebLinkError> {
    let metadata = match std::fs::symlink_metadata(link_path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Err(WebLinkError::unlink_not_linked(package_name));
        }
        Err(error) => return Err(WebLinkError::filesystem(error)),
    };
    if !metadata.file_type().is_symlink() {
        return Err(WebLinkError::not_package_link(package_name));
    }

    #[cfg(unix)]
    {
        std::fs::remove_file(link_path).map_err(WebLinkError::filesystem)
    }

    #[cfg(windows)]
    {
        use std::os::windows::fs::FileTypeExt;

        let result = if metadata.file_type().is_symlink_dir() {
            std::fs::remove_dir(link_path)
        } else {
            std::fs::remove_file(link_path)
        };
        result.map_err(WebLinkError::filesystem)
    }

    #[cfg(not(any(unix, windows)))]
    std::fs::remove_file(link_path).map_err(WebLinkError::filesystem)
}
