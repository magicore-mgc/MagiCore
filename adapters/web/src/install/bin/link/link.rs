//! Create the platform-native link or command shim for a package binary.
//! Tạo symlink hoặc command shim phù hợp cho binary package.

use mgc_types::{MgError, MgResult};
use std::path::Path;

use super::super::error::BinError;

pub fn create_bin_link(link: &Path, target: &Path) -> MgResult<()> {
    create_bin_link_inner(link, target).map_err(MgError::from)
}

#[cfg(unix)]
fn create_bin_link_inner(link: &Path, target: &Path) -> Result<(), BinError> {
    use std::os::unix::fs::symlink;

    if link.exists() {
        std::fs::remove_file(link).map_err(|err| {
            BinError::Link(MgError::Other(format!(
                "failed to remove existing bin link '{}': {}",
                link.display(),
                err
            )))
        })?;
    }

    symlink(target, link).map_err(|err| {
        BinError::Link(MgError::Other(format!(
            "failed to create bin link '{}' -> '{}': {}",
            link.display(),
            target.display(),
            err
        )))
    })?;
    Ok(())
}

#[cfg(not(unix))]
fn create_bin_link_inner(link: &Path, target: &Path) -> Result<(), BinError> {
    // npm-style .cmd shim: Node scripts need the Node runtime on Windows.
    // Shim .cmd kiểu npm: script Node cần Node runtime trên Windows.
    let command = format!(
        "@echo off\r\n\
         where node >NUL 2>NUL || (echo node not found on PATH — cannot run {} 1>&2 & exit /b 127)\r\n\
         @node \"{}\" %*\r\n",
        link.display(),
        target.display()
    );
    std::fs::write(link.with_extension("cmd"), command).map_err(|err| {
        BinError::Link(MgError::Other(format!(
            "failed to create cmd shim for '{}' -> '{}': {}",
            link.display(),
            target.display(),
            err
        )))
    })?;
    Ok(())
}
