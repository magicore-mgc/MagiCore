//! Builds the PATH used for guarded process execution.
//! Tạo PATH dùng cho tiến trình được bảo vệ.

use std::ffi::OsString;
use std::path::Path;

use super::super::EnvironmentError;

pub(in crate::run) fn is_path_env_key(key: &str) -> bool {
    #[cfg(windows)]
    {
        key.eq_ignore_ascii_case("PATH")
    }
    #[cfg(not(windows))]
    {
        key == "PATH"
    }
}

pub(in crate::run) fn guarded_path_env(
    shadow_dir: &Path,
    explicit_env: &[(String, String)],
) -> std::result::Result<OsString, EnvironmentError> {
    let explicit_path = explicit_env
        .iter()
        .rev()
        .find(|(key, _)| is_path_env_key(key))
        .map(|(_, value)| OsString::from(value));
    let base_path = explicit_path.or_else(|| std::env::var_os("PATH"));
    let mut paths = Vec::new();
    paths.push(shadow_dir.to_path_buf());
    if let Some(path) = base_path {
        paths.extend(std::env::split_paths(&path));
    }
    std::env::join_paths(paths).map_err(EnvironmentError::JoinPaths)
}
