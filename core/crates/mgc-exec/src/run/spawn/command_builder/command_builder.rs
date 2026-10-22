//! Construct a child command and configure its stdio without spawning it.
//! Tạo cấu hình child command và stdio nhưng không tự spawn process.

use super::super::super::super::ExecOptions;
#[cfg(windows)]
use super::super::super::super::is_path_env_key;
use super::super::super::super::output::OutputMode;
#[cfg(windows)]
use super::super::super::windows_command::{resolve_windows_shim, windows_batch_command};
use anyhow::Result;
#[cfg(windows)]
use std::ffi::OsString;
use std::path::Path;
use std::process::Command;

/// Build the platform-specific command while retaining caller argv and cwd.
/// Dựng lệnh theo nền tảng và giữ nguyên argv, cwd do caller truyền.
pub(in crate::run::implementation::spawn::implementation) fn build_command(
    cmd: &str,
    args: &[String],
    cwd: &Path,
    _opts: &ExecOptions,
    mode: OutputMode,
) -> Result<Command> {
    // Windows std does not resolve `.cmd`/`.bat` through PATH; use caller PATH for safe shim lookup.
    // Windows std không resolve shim `.cmd`/`.bat` qua PATH; tra theo PATH do caller truyền.
    #[cfg(windows)]
    let path_var: Option<OsString> = _opts
        .env
        .iter()
        .rev()
        .find(|(key, _)| is_path_env_key(key))
        .map(|(_, value)| OsString::from(value));
    #[cfg(windows)]
    let resolved_cmd = resolve_windows_shim(cmd, path_var.as_deref());
    #[cfg(unix)]
    let resolved_cmd: &str = cmd;
    #[cfg(all(not(unix), not(windows)))]
    let resolved_cmd: &str = cmd;

    // Batch shims go through the fixed, validated system interpreter.
    // Shim batch đi qua interpreter hệ thống cố định sau khi validate.
    #[cfg(windows)]
    let mut command = {
        let resolved_str = resolved_cmd.to_string_lossy();
        let is_script = resolved_str.to_ascii_lowercase().ends_with(".cmd")
            || resolved_str.to_ascii_lowercase().ends_with(".bat");

        if is_script {
            windows_batch_command(&resolved_str, args, cwd)?
        } else {
            let mut command = Command::new(&resolved_cmd);
            command.args(args).current_dir(cwd);
            command
        }
    };

    #[cfg(all(not(unix), not(windows)))]
    let mut command = {
        let mut command = Command::new(resolved_cmd);
        command.args(args).current_dir(cwd);
        command
    };

    #[cfg(unix)]
    let mut command = {
        let mut command = Command::new(resolved_cmd);
        command.args(args).current_dir(cwd);
        command
    };

    match mode {
        OutputMode::Capture => {
            command
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped());
        }
        OutputMode::Inherit => {
            command
                .stdin(std::process::Stdio::inherit())
                .stdout(std::process::Stdio::inherit())
                .stderr(std::process::Stdio::inherit());
        }
    }
    Ok(command)
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod test;
