//! `npm_probe.rs` — exact allowlisted npm probe classification.
//! Phân loại chính xác probe npm được phép.

use super::super::super::super::super::{
    is_generated_shadow_directory, is_npm_version_shell_probe, process_basename,
};
use super::{
    generated_npm_probe_shim_from_command_line, is_verified_shadow_npm_probe_shim,
    npm_probe_command_line_matches_shim_path,
};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

pub(in crate::run::implementation::process_scan::monitor::implementation) fn is_shadow_npm_version_probe(
    command: &[u8],
    command_line: &[u8],
    shadow_dir: &Path,
) -> bool {
    use std::os::unix::ffi::OsStrExt;

    let shim_path = shadow_dir.join("npm");
    if !is_verified_shadow_npm_probe_shim(shadow_dir) {
        return false;
    }

    let process = process_basename(&String::from_utf8_lossy(command));
    npm_probe_command_line_matches_shim_path(
        &process,
        command_line,
        shim_path.as_os_str().as_bytes(),
    )
}

pub(in crate::run::implementation::process_scan::monitor::implementation) fn is_allowed_shadow_npm_version_probe(
    command: &[u8],
    command_line: &[u8],
    own_shadow_dir: &Path,
    forbidden_name: &str,
) -> bool {
    forbidden_name == "npm"
        && (is_shadow_npm_version_probe(command, command_line, own_shadow_dir)
            || is_generated_shadow_npm_version_probe(command, command_line)
            || (is_verified_shadow_npm_probe_shim(own_shadow_dir)
                && is_npm_version_shell_probe(command, command_line)))
}

pub(in crate::run::implementation::process_scan::monitor::implementation) fn is_generated_shadow_npm_version_probe(
    command: &[u8],
    command_line: &[u8],
) -> bool {
    use std::os::unix::ffi::OsStringExt;

    let process = process_basename(&String::from_utf8_lossy(command));
    let Some(shim_bytes) = generated_npm_probe_shim_from_command_line(&process, command_line)
    else {
        return false;
    };
    let shim_path = PathBuf::from(OsString::from_vec(shim_bytes));
    if !shim_path.file_name().is_some_and(|name| name == "npm") {
        return false;
    }
    let Some(parent) = shim_path.parent() else {
        return false;
    };
    if !is_generated_shadow_directory(parent) {
        return false;
    }
    let (Ok(directory_metadata), Ok(shim_metadata)) = (
        std::fs::symlink_metadata(parent),
        std::fs::symlink_metadata(&shim_path),
    ) else {
        return false;
    };
    if !directory_metadata.file_type().is_dir()
        || !shim_metadata.file_type().is_file()
        || directory_metadata.file_type().is_symlink()
        || shim_metadata.file_type().is_symlink()
    {
        return false;
    }
    is_shadow_npm_version_probe(command, command_line, parent)
}
