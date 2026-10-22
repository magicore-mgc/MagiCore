//! `verified_shim.rs` — exact executor npm shim verification.
//! Xác minh chính xác npm shim của executor.

use super::super::super::super::super::super::NPM_VERSION_PROBE_SHIM_CONTENT;
use std::path::Path;

#[cfg(unix)]
pub(in crate::run::implementation::process_scan::monitor::implementation) fn is_verified_shadow_npm_probe_shim(
    shadow_dir: &Path,
) -> bool {
    use std::os::unix::fs::PermissionsExt;

    let Ok(directory_metadata) = std::fs::metadata(shadow_dir) else {
        return false;
    };
    let shim_path = shadow_dir.join("npm");
    let Ok(shim_metadata) = std::fs::metadata(&shim_path) else {
        return false;
    };
    let Ok(shim_content) = std::fs::read(&shim_path) else {
        return false;
    };
    directory_metadata.permissions().mode() & 0o222 == 0
        && shim_metadata.permissions().mode() & 0o222 == 0
        && shim_content == NPM_VERSION_PROBE_SHIM_CONTENT.as_bytes()
}
