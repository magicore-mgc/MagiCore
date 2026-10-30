//! Guarded environment owner wiring and reexports.
//! Wiring và reexport các owner môi trường được bảo vệ.

#[path = "error.rs"]
mod error;
pub(in crate::run) use error::EnvironmentError;
#[path = "types.rs"]
mod types;
pub(in crate::run) use types::ShadowPath;
#[path = "deny_shim/mod.rs"]
mod deny_shim;
#[cfg(unix)]
pub(in crate::run) use deny_shim::NPM_VERSION_PROBE_SHIM_CONTENT;
pub(in crate::run) use deny_shim::write_blocker;

#[path = "shadow_path/mod.rs"]
mod shadow_path;
#[cfg(unix)]
pub(in crate::run) use shadow_path::is_generated_shadow_directory;
#[cfg(any(windows, test))]
pub(in crate::run) use shadow_path::unique_temp_dir;
#[path = "guarded_path/mod.rs"]
mod guarded_path;
pub(in crate::run) use guarded_path::{guarded_path_env, is_path_env_key};
#[cfg(windows)]
#[path = "windows_executable_identity/mod.rs"]
mod windows_executable_identity;
#[cfg(windows)]
pub(in crate::run::implementation) use windows_executable_identity::{
    windows_current_executable_sha256, windows_sha256_file,
};

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
