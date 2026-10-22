//! Authenticate the native MGC npm version-probe shim on Windows.
//! Xác thực shim npm --version native của MGC trên Windows.

use super::super::super::super::ShadowPath;
use super::super::super::super::process_scan::WindowsProcessSnapshot;
#[cfg(all(windows, test))]
use super::super::super::super::unique_temp_dir;
use super::error::NpmProbeAuthenticationError;
use std::path::{Path, PathBuf};

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;

#[cfg(windows)]
use super::super::super::super::{windows_current_executable_sha256, windows_sha256_file};

#[cfg(all(windows, test))]
pub(super) fn is_shadow_windows_npm_version_probe(
    process: &WindowsProcessSnapshot,
    executable_path: &Path,
    shadow_path: &ShadowPath,
) -> bool {
    windows_npm_probe_authentication(process, Some(executable_path), shadow_path).is_ok()
}

#[cfg(windows)]
pub(super) fn windows_npm_probe_authentication(
    process: &WindowsProcessSnapshot,
    executable_path: Option<&Path>,
    shadow_path: &ShadowPath,
) -> std::result::Result<(), NpmProbeAuthenticationError> {
    use std::ffi::OsStr;

    let command_shape_is_safe = match process.command.as_slice() {
        // Windows can expose the process image before arguments become visible.
        // A one-argument snapshot is accepted only while argv[0] still names npm.
        // (Windows có thể lộ image trước args. Snapshot một argv chỉ an toàn
        // nếu argv[0] vẫn nhận diện alias npm.)
        [program] => windows_npm_probe_argv0_path(program, shadow_path.path()).is_some(),
        [_, version] => version.as_os_str() == OsStr::new("--version"),
        _ => false,
    };
    if !process.image_name.eq_ignore_ascii_case("npm.exe") {
        return Err(NpmProbeAuthenticationError::ImageNameMismatch);
    }
    if !command_shape_is_safe {
        return Err(NpmProbeAuthenticationError::UnsafeCommandShape);
    }
    let expected_digest = windows_current_executable_sha256()
        .map_err(|_| NpmProbeAuthenticationError::ExecutableDigestUnavailable)?;
    // Windows can report the hard-linked target path instead of the npm alias.
    // Authenticate the actual process image bytes and safe command shape rather
    // than depending on which hard-link path the OS reports.
    // (Windows có thể báo path đích hard-link thay vì alias npm. Xác thực byte
    // image thực tế và args an toàn, không phụ thuộc OS trả path nào.)
    let executable_path =
        executable_path.ok_or(NpmProbeAuthenticationError::ImagePathUnavailable)?;
    let image_digest = windows_sha256_file(executable_path)
        .map_err(|_| NpmProbeAuthenticationError::ImageDigestUnavailable)?;
    if image_digest != expected_digest {
        return Err(NpmProbeAuthenticationError::ImageDigestMismatch);
    }
    Ok(())
}

#[cfg(all(test, not(windows)))]
pub(in crate::run::implementation::windows_job::inspection) fn windows_npm_probe_authentication(
    _process: &WindowsProcessSnapshot,
    _executable_path: Option<&Path>,
    _shadow_path: &ShadowPath,
) -> std::result::Result<(), NpmProbeAuthenticationError> {
    Err(NpmProbeAuthenticationError::UnsupportedPlatform)
}

#[cfg(any(windows, test))]
pub(in crate::run::implementation::windows_job::inspection) fn windows_npm_probe_argv0_path(
    program: &std::ffi::OsStr,
    shadow_dir: &Path,
) -> Option<PathBuf> {
    use std::ffi::OsStr;

    if program.eq_ignore_ascii_case(OsStr::new("npm"))
        || program.eq_ignore_ascii_case(OsStr::new("npm.exe"))
    {
        return Some(shadow_dir.join("npm.exe"));
    }
    let path = Path::new(program);
    (path.is_absolute()
        && path
            .file_name()
            .is_some_and(|name| name.eq_ignore_ascii_case(OsStr::new("npm.exe"))))
    .then(|| path.to_path_buf())
}
