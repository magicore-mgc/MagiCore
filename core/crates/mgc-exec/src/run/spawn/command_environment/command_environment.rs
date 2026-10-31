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
    // Inherit the caller environment FIRST so the layering below only decides
    // precedence, never whether the caller's PORT/NODE_ENV/DATABASE_URL/API keys
    // exist. Only an explicit `clean_env` request scrubs afterwards.
    // Kế thừa environment caller TRƯỚC để phần xếp chồng bên dưới chỉ quyết định
    // thứ tự ưu tiên, không quyết định việc PORT/NODE_ENV/DATABASE_URL/API key của
    // caller có tồn tại hay không. Chỉ khi caller yêu cầu `clean_env` mới scrub sau.
    inherit_caller_environment(command);

    for (key, value) in &opts.env {
        if !is_path_env_key(key) {
            // An explicit assignment always wins over the inherited value, and
            // removing first also strips an inherited duplicate so MagiCore vars
            // replace — never shadow — the caller's copy.
            // Giá trị explicit luôn thắng giá trị kế thừa; xóa trước để biến của
            // MagiCore thay thế — không che — bản sao của caller.
            command.env_remove(key);
            command.env(key, value);
        }
    }
    command.env("PATH", path_env);

    // Without `clean_env`, every inherited variable is preserved as-is.
    // Không bật `clean_env` thì giữ nguyên mọi biến kế thừa.
    if !opts.clean_env {
        return Ok(());
    }

    // `clean_env` keeps only variables the executor owns plus the platform
    // variables a child cannot start without. Everything else inherited is
    // dropped, exactly as before this change. `PATH` is executor-owned, so its
    // filtering happens below instead of in the explicit-assignment loop.
    // `clean_env` chỉ giữ biến do executor sở hữu và biến hệ thống mà child không
    // thể khởi động nếu thiếu. Mọi biến kế thừa khác bị bỏ như trước thay đổi này.
    // `PATH` do executor sở hữu, nên nó được lọc bên dưới.
    let managed_keys = opts
        .env
        .iter()
        .map(|(key, _)| key.as_str())
        .chain(std::iter::once("PATH"))
        .collect::<Vec<_>>();
    // Native Windows children need these runtime paths even after `env_clear()`.
    // (Child Windows cần các đường dẫn runtime này kể cả sau `env_clear()`.)
    #[cfg(not(unix))]
    let critical_keys: &[&str] = &[
        "SYSTEMROOT",
        "WINDIR",
        "TEMP",
        "TMP",
        "USERPROFILE",
        "APPDATA",
        "LOCALAPPDATA",
        "ProgramData",
    ];
    #[cfg(unix)]
    let critical_keys: &[&str] = &[];
    keep_only(command, &managed_keys, critical_keys);
    Ok(())
}

/// Copy the caller process environment into a command that starts empty.
/// Sao chép environment của process caller sang command bắt đầu từ rỗng.
fn inherit_caller_environment(command: &mut Command) {
    command.envs(std::env::vars_os());
}

/// Drop inherited variables outside the managed/critical sets.
/// Bỏ biến kế thừa nằm ngoài tập managed/critical.
fn keep_only(command: &mut Command, managed_keys: &[&str], critical_keys: &[&str]) {
    let env = command
        .get_envs()
        .map(|(key, value)| (key.to_os_string(), value.map(|value| value.to_os_string())))
        .collect::<Vec<_>>();
    // Rebuild the environment so the allow decision is made exactly once.
    // Dựng lại environment để quyết định allow chỉ được xét đúng một lần.
    command.env_clear();
    for (key, value) in env {
        let Some(value) = value else {
            continue;
        };
        let name = key.to_string_lossy();
        let managed = managed_keys
            .iter()
            .any(|managed| is_same_env_key(&name, managed));
        let critical = critical_keys
            .iter()
            .any(|critical| is_same_env_key(&name, critical));
        if managed || critical {
            command.env(key, value);
        }
    }
}

/// Compare environment names with Windows' case-insensitive semantics.
/// So sánh tên biến với ngữ nghĩa không phân biệt hoa thường của Windows.
fn is_same_env_key(left: &str, right: &str) -> bool {
    #[cfg(windows)]
    {
        left.eq_ignore_ascii_case(right)
    }
    #[cfg(not(windows))]
    {
        left == right
    }
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
