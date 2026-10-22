//! Environment facade — export the guarded execution environment owner.
//! Facade môi trường — export owner quản lý môi trường thực thi có bảo vệ.

#[path = "environment.rs"]
mod implementation;
pub(in crate::run) use implementation::{
    EnvironmentError, ShadowPath, guarded_path_env, is_path_env_key,
};
#[cfg(unix)]
pub(in crate::run) use implementation::{
    NPM_VERSION_PROBE_SHIM_CONTENT, is_generated_shadow_directory,
};
#[cfg(windows)]
pub(super) use implementation::{windows_current_executable_sha256, windows_sha256_file};
