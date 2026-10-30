//! Finds fixed toolchain entries and probes versions through the MGC executor.
//! Tìm toolchain cố định và truy vấn version qua executor của MGC.

use std::env;
#[cfg(test)]
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::time::Duration;

use mgc_exec::allowlist::ExecutionScope;
use mgc_exec::run::{ExecOptions, run};

use super::super::core_toolchain::requirements_for_core;
use super::super::model::Tool;

const VERSION_PROBE_TIMEOUT: Duration = Duration::from_secs(5);
pub(in crate::commands::doctor) const MAX_VERSION_LINE_BYTES: usize = 256;
pub(in crate::commands::doctor) const VERSION_TRUNCATION_MARKER: &str = "...";
pub(in crate::commands::doctor) const VERSION_UNAVAILABLE: &str = "?";
/// Environment keys needed by common toolchain shims; runtime hooks are excluded.
/// Biến cần cho shim toolchain phổ biến; loại startup hook của runtime.
pub(in crate::commands::doctor) const VERSION_PROBE_ENV_KEYS: &[&str] = &[
    "HOME",
    "CARGO_HOME",
    "RUSTUP_HOME",
    "RUSTUP_TOOLCHAIN",
    "ASDF_DATA_DIR",
    "MISE_DATA_DIR",
    "PYENV_ROOT",
    "VOLTA_HOME",
];

/// Closed inventory and fixed, read-only version arguments in display order.
/// Danh sách đóng và đối số version chỉ-đọc cố định theo thứ tự hiển thị.
pub(in crate::commands::doctor) const TOOLCHAIN_PROBES: [(&str, &str, &[&str]); 6] = [
    ("node", "node", &["--version"]),
    ("python3", "python3", &["-S", "--version"]),
    ("rustc", "rustc", &["-vV"]),
    ("cargo", "cargo", &["--version"]),
    ("git", "git", &["--version"]),
    ("docker", "docker", &["--version"]),
];

pub(in crate::commands::doctor) fn inventory() -> Vec<Tool> {
    TOOLCHAIN_PROBES
        .iter()
        .map(|&(name, binary, args)| tool(name, binary, args, None))
        .collect()
}

/// Probe the general tools plus the core-specific tools when a core is known.
/// Probe tool chung cộng tool riêng của core khi đã biết core.
///
/// Core-only tools are appended after the general inventory and de-duplicated:
/// a tool already present from the general list gains a `required_by_core` tag
/// instead of being probed twice. Repeated calls (one per core requirement) are
/// avoided by keying on the executable basename.
/// Tool riêng core được nối sau inventory chung và khử trùng: tool đã có từ
/// danh sách chung được gắn `required_by_core` thay vì probe hai lần. Khử trùng
/// theo basename executable.
pub(in crate::commands::doctor) fn inventory_for_core(core: &str) -> Vec<Tool> {
    let mut tools = inventory();
    for requirement in requirements_for_core(core) {
        // Skip a core tool already covered by the general inventory row.
        // Bỏ tool core đã được hàng inventory chung bao phủ.
        if let Some(existing) = tools
            .iter_mut()
            .find(|tool| tool.name == requirement.binary)
        {
            if existing.required_by_core.is_none() {
                existing.required_by_core = Some(requirement.core);
            }
            continue;
        }
        tools.push(tool(
            requirement.binary,
            requirement.binary,
            requirement.probe_args,
            Some(requirement.core),
        ));
    }
    tools
}

fn which(bin: &str) -> Option<PathBuf> {
    let path = env::var_os("PATH")?;
    let candidate = which_in_paths(bin, env::split_paths(path.as_os_str()))?;
    pin_candidate(&candidate)
}

/// Search OS-parsed PATH entries in order, skipping empty entries and directories.
/// Tìm PATH theo API của OS, giữ thứ tự và bỏ entry rỗng hoặc candidate là thư mục.
#[cfg(test)]
pub(in crate::commands::doctor) fn which_in_path(bin: &str, path: &str) -> Option<PathBuf> {
    which_in_paths(bin, env::split_paths(OsStr::new(path)))
}

/// Return the first regular-file candidate from an OS path iterator.
/// Trả candidate file thường đầu tiên từ iterator đường dẫn của OS.
pub(in crate::commands::doctor) fn which_in_paths(
    bin: &str,
    paths: impl IntoIterator<Item = PathBuf>,
) -> Option<PathBuf> {
    for dir in paths {
        if dir.as_os_str().is_empty() {
            continue;
        }
        let candidate = dir.join(bin);
        if candidate.is_file() {
            return Some(candidate);
        }
        #[cfg(windows)]
        {
            let candidate_exe = dir.join(format!("{bin}.exe"));
            if candidate_exe.is_file() {
                return Some(candidate_exe);
            }
        }
    }
    None
}

/// Pin the executable leaf to its canonical absolute parent directory.
/// Ghim leaf executable vào parent tuyệt đối đã canonicalize.
pub(in crate::commands::doctor) fn pin_candidate(candidate: &Path) -> Option<PathBuf> {
    let file_name = candidate.file_name()?;
    let parent = candidate
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let absolute_candidate = parent.canonicalize().ok()?.join(file_name);
    absolute_candidate.is_file().then_some(absolute_candidate)
}

/// Escape terminal control characters without changing ordinary path separators.
/// Escape ký tự điều khiển terminal nhưng giữ nguyên dấu phân cách path thông thường.
pub(in crate::commands::doctor) fn safe_path_display(path: &Path) -> String {
    let mut displayed = String::new();
    for character in path.to_string_lossy().chars() {
        if character.is_control() {
            displayed.extend(character.escape_default());
        } else {
            displayed.push(character);
        }
    }
    displayed
}

fn tool_version(path: &Path, probe_args: &[&str]) -> String {
    // Recheck the selected candidate immediately before invoking MGC. Passing
    // this concrete path prevents a second PATH lookup in the child process.
    // Gọi lại kiểm tra candidate ngay trước khi chạy qua MGC; dùng path cụ thể
    // để process con không tra PATH lần thứ hai.
    let Ok(canonical) = path.canonicalize() else {
        return VERSION_UNAVAILABLE.into();
    };
    if !canonical.is_file() || !path.is_file() {
        return VERSION_UNAVAILABLE.into();
    }
    let Some(command) = path.to_str() else {
        return VERSION_UNAVAILABLE.into();
    };

    let args = probe_args
        .iter()
        .map(|argument| (*argument).to_string())
        .collect::<Vec<_>>();
    let options = version_probe_options();
    run(command, &args, &options)
        .map(|report| version_from_stdout(&report.stdout_tail))
        .unwrap_or_else(|_| VERSION_UNAVAILABLE.into())
}

pub(in crate::commands::doctor) fn version_probe_options() -> ExecOptions {
    ExecOptions {
        timeout: Some(VERSION_PROBE_TIMEOUT),
        execution_scope: Some(ExecutionScope::BuildRunner),
        clean_env: true,
        env: version_probe_environment(),
        ..ExecOptions::default()
    }
}

/// Preserve only home/toolchain-manager selectors; omit runtime startup hooks.
/// Chỉ giữ biến HOME/toolchain manager; bỏ hook startup của runtime.
pub(in crate::commands::doctor) fn version_probe_environment() -> Vec<(String, String)> {
    env::vars_os()
        .filter_map(|(key, value)| {
            let key = key.to_str()?;
            if !VERSION_PROBE_ENV_KEYS
                .iter()
                .any(|allowed| key.eq_ignore_ascii_case(allowed))
            {
                return None;
            }
            Some((key.to_string(), value.into_string().ok()?))
        })
        .collect()
}

/// Extract a bounded, terminal-safe version line from executor output.
/// Trích một dòng version an toàn, có giới hạn từ output của executor.
pub(in crate::commands::doctor) fn version_from_stdout(stdout: &str) -> String {
    let first_line = stdout.trim().lines().next().unwrap_or_default();
    let safe_line = first_line
        .chars()
        .filter(|character| !character.is_control())
        .collect::<String>();
    let safe_line = safe_line.trim();
    if safe_line.is_empty() {
        return VERSION_UNAVAILABLE.into();
    }

    let truncated = safe_line.len() > MAX_VERSION_LINE_BYTES;
    let content_limit = if truncated {
        MAX_VERSION_LINE_BYTES - VERSION_TRUNCATION_MARKER.len()
    } else {
        MAX_VERSION_LINE_BYTES
    };
    let mut end = safe_line.len().min(content_limit);
    while !safe_line.is_char_boundary(end) {
        end -= 1;
    }
    let mut version = safe_line[..end].to_string();
    if truncated {
        version.push_str(VERSION_TRUNCATION_MARKER);
    }
    version
}

fn tool(
    name: &'static str,
    bin: &str,
    probe_args: &[&str],
    required_by_core: Option<&'static str>,
) -> Tool {
    match which(bin) {
        Some(path) => {
            let version = tool_version(&path, probe_args);
            Tool {
                name,
                path: Some(safe_path_display(&path)),
                version,
                required_by_core,
            }
        }
        None => Tool {
            name,
            path: None,
            version: "MISSING".into(),
            required_by_core,
        },
    }
}
