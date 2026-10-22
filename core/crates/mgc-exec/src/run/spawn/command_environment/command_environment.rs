//! Apply explicit environment and guarded PATH to a child command.
//! Áp environment explicit và guarded PATH cho child command.

use crate::run::implementation::{
    EnvironmentError, ExecOptions, guarded_path_env, is_path_env_key,
};
use std::path::Path;
use std::process::Command;

/// Configure an existing command without resolving or launching it.
/// Cấu hình command hiện có nhưng không resolve hoặc launch.
pub(in crate::run::implementation) fn configure_command_environment(
    command: &mut Command,
    opts: &ExecOptions,
    shadow_dir: &Path,
) -> std::result::Result<(), EnvironmentError> {
    let path_env = guarded_path_env(shadow_dir, &opts.env)?;
    if opts.clean_env {
        command.env_clear();
        for (key, value) in &opts.env {
            if !is_path_env_key(key) {
                command.env(key, value);
            }
        }
        command.env("PATH", path_env);
        // Restore these after env_clear() so native Windows children retain required runtime paths.
        // Đặt lại sau env_clear() để child Windows còn các đường dẫn runtime bắt buộc.
        #[cfg(not(unix))]
        for critical_var in [
            "SYSTEMROOT",
            "WINDIR",
            "TEMP",
            "TMP",
            "USERPROFILE",
            "APPDATA",
            "LOCALAPPDATA",
            "ProgramData",
        ] {
            if opts
                .env
                .iter()
                .any(|(key, _)| key.eq_ignore_ascii_case(critical_var))
            {
                continue;
            }
            if let Some(value) = std::env::var_os(critical_var) {
                command.env(critical_var, value);
            }
        }
    } else {
        // Preserve caller environment and place deny shims before caller PATH.
        // Giữ environment caller và đặt deny shim trước PATH caller.
        for (key, value) in &opts.env {
            if !is_path_env_key(key) {
                command.env(key, value);
            }
        }
        command.env("PATH", path_env);
    }
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
