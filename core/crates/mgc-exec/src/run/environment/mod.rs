//! Environment facade — export the guarded execution environment owner.
//! Facade môi trường — export owner quản lý môi trường thực thi có bảo vệ.

#[path = "environment.rs"]
mod implementation;
pub(in crate::run) use implementation::{
    EnvironmentError, ShadowPath, guarded_path_env, is_path_env_key,
};
#[cfg(unix)]
pub(in crate::run::implementation) use implementation::{
    NPM_VERSION_PROBE_SHIM_CONTENT, is_generated_shadow_directory,
};
// The env-owner's inner re-export is scoped to `environment`'s own impl module,
// so the facade must forward the temp-dir helper to serve `run::implementation`.
// (Re-export bên trong owner env chỉ giới hạn ở impl module của `environment`,
// nên facade phải chuyển tiếp helper temp-dir để `run::implementation` dùng được.)
#[cfg(any(windows, test))]
pub(in crate::run::implementation) use implementation::unique_temp_dir;
#[cfg(windows)]
pub(in crate::run::implementation) use implementation::{
    windows_current_executable_sha256, windows_sha256_file,
};
