//! Inspect or set executable permissions using the host platform behavior.
//! Đọc hoặc đặt quyền executable theo hành vi của OS hiện tại.

use mgc_types::{MgError, MgResult};
use std::path::Path;

use super::super::error::BinError;

pub fn is_executable(path: &Path) -> MgResult<bool> {
    is_executable_inner(path).map_err(MgError::from)
}

#[cfg(unix)]
fn is_executable_inner(path: &Path) -> Result<bool, BinError> {
    use std::os::unix::fs::PermissionsExt;

    let mode = std::fs::metadata(path)
        .map_err(|error| BinError::Executable(MgError::Io(error)))?
        .permissions()
        .mode();
    Ok(mode & 0o111 != 0)
}

#[cfg(not(unix))]
fn is_executable_inner(_: &Path) -> Result<bool, BinError> {
    Ok(false)
}

pub fn set_executable(path: &Path, executable: bool) -> MgResult<()> {
    set_executable_inner(path, executable).map_err(MgError::from)
}

#[cfg(unix)]
fn set_executable_inner(path: &Path, executable: bool) -> Result<(), BinError> {
    use std::os::unix::fs::PermissionsExt;

    let mut perms = std::fs::metadata(path)
        .map_err(|error| BinError::Executable(MgError::Io(error)))?
        .permissions();
    let mode = if executable { 0o755 } else { 0o644 };
    perms.set_mode(mode);
    std::fs::set_permissions(path, perms)
        .map_err(|error| BinError::Executable(MgError::Io(error)))?;
    Ok(())
}

#[cfg(not(unix))]
fn set_executable_inner(_: &Path, _: bool) -> Result<(), BinError> {
    Ok(())
}
