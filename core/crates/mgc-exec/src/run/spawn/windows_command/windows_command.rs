//! In-process Windows command resolution and safe batch dispatch.
//! Resolve lệnh Windows trong process và chạy batch an toàn.

#[cfg(any(windows, test))]
use super::error::WindowsCommandError;
#[cfg(any(windows, test))]
use std::ffi::{OsStr, OsString};
#[cfg(any(windows, test))]
#[cfg(windows)]
use std::path::Path;
#[cfg(windows)]
use std::path::PathBuf;

/// Find a bare Windows command in the supplied PATH, preferring native PE
/// executables over command shims and extensionless scripts.
/// (Tìm executable Windows trong PATH được truyền, ưu tiên PE trước shim và script không đuôi.)
#[cfg(any(windows, test))]
pub(in crate::run::implementation) fn find_windows_command(
    cmd: &str,
    search_path: Option<&OsStr>,
) -> Option<OsString> {
    let search_path = search_path
        .map(OsStr::to_os_string)
        .or_else(|| std::env::var_os("PATH"))?;
    let suffixes = [".exe", ".com", ".cmd", ".bat", ""];
    let mut matches: [Option<OsString>; 5] = Default::default();

    for directory in std::env::split_paths(&search_path) {
        // An empty PATH entry resolves against the current directory. Do not
        // turn an absent/empty path entry into an implicit cwd search.
        if directory.as_os_str().is_empty() {
            continue;
        }
        for (rank, suffix) in suffixes.iter().enumerate() {
            if matches[rank].is_some() {
                continue;
            }
            let candidate = directory.join(format!("{cmd}{suffix}"));
            if candidate.is_file() {
                matches[rank] = Some(candidate.into_os_string());
            }
        }
    }

    matches.into_iter().flatten().next()
}

/// Resolve a bare command name to a Windows shim without spawning a helper.
/// PATH is the caller-provided value, including project-local binary folders.
/// (Resolve shim trong process theo PATH caller truyền, gồm bin cục bộ của project.)
#[cfg(windows)]
pub(in crate::run::implementation) fn resolve_windows_shim(
    cmd: &str,
    search_path: Option<&OsStr>,
) -> OsString {
    if cmd.contains('.') || cmd.contains('\\') || cmd.contains('/') {
        return OsString::from(cmd);
    }
    find_windows_command(cmd, search_path).unwrap_or_else(|| OsString::from(cmd))
}

/// Convert canonical Windows drive/UNC paths without losing UTF-16 data.
/// Bỏ prefix verbatim ở drive/UNC mà không làm mất dữ liệu UTF-16.
#[cfg(any(windows, test))]
pub(in crate::run::implementation) fn windows_tool_path_units(path: &[u16]) -> Vec<u16> {
    let unc_prefix = r"\\?\UNC\".encode_utf16().collect::<Vec<_>>();
    if let Some(tail) = path.strip_prefix(unc_prefix.as_slice()) {
        return r"\\".encode_utf16().chain(tail.iter().copied()).collect();
    }
    let disk_prefix = r"\\?\".encode_utf16().collect::<Vec<_>>();
    if let Some(tail) = path.strip_prefix(disk_prefix.as_slice())
        && tail.len() >= 3
        && ((b'A' as u16..=b'Z' as u16).contains(&tail[0])
            || (b'a' as u16..=b'z' as u16).contains(&tail[0]))
        && tail[1] == b':' as u16
        && tail[2] == b'\\' as u16
    {
        return tail.to_vec();
    }
    path.to_vec()
}

/// Keep untrusted batch arguments outside cmd.exe expansion and operator syntax.
/// Chặn args không tin cậy khỏi cú pháp expansion/toán tử của cmd.exe.
#[cfg(any(windows, test))]
pub(in crate::run::implementation) fn validate_windows_batch_invocation(
    script_path: &str,
    args: &[String],
) -> Result<(), WindowsCommandError> {
    let contains_cmd_metacharacter = |value: &str| {
        value.chars().any(|character| {
            matches!(
                character,
                '"' | '%' | '!' | '&' | '|' | '<' | '>' | '^' | '(' | ')' | '\n' | '\r'
            )
        })
    };
    if contains_cmd_metacharacter(script_path)
        || args.iter().any(|arg| contains_cmd_metacharacter(arg))
    {
        return Err(WindowsCommandError::UnsupportedBatchSyntax);
    }
    Ok(())
}

/// Resolve system executables only below System32, never through caller PATH.
/// Chỉ resolve executable hệ thống trong System32, không dò PATH của caller.
#[cfg(windows)]
pub(in crate::run::implementation) fn windows_system_tool_path(
    name: &str,
) -> Result<PathBuf, WindowsCommandError> {
    let system_root =
        std::env::var_os("SystemRoot").ok_or(WindowsCommandError::SystemRootUnavailable)?;
    let path = PathBuf::from(system_root).join("System32").join(name);
    if !path.is_absolute() || !path.is_file() {
        return Err(WindowsCommandError::SystemToolUnavailable);
    }
    Ok(path)
}

/// Construct the fixed-interpreter invocation for a validated batch shim.
/// Tạo lệnh gọi qua interpreter cố định cho shim batch đã được kiểm tra.
#[cfg(windows)]
pub(super) fn windows_batch_command(
    script_path: &str,
    args: &[String],
    cwd: &Path,
) -> Result<std::process::Command, WindowsCommandError> {
    // Normalize only the interpreter argument; keep canonical paths for validation.
    // Chỉ chuẩn hóa path đưa vào shell; giữ canonical path cho bước kiểm tra.
    // `cmd.exe` does not accept the extended-length prefix produced by canonicalize().
    // `cmd.exe` không nhận prefix extended-length mà canonicalize() có thể trả về.
    let cmd_path = if let Some(unc) = script_path.strip_prefix(r"\\?\UNC\") {
        format!(r"\\{unc}")
    } else if let Some(local) = script_path.strip_prefix(r"\\?\") {
        local.to_string()
    } else {
        script_path.to_string()
    };
    validate_windows_batch_invocation(&cmd_path, args)?;

    // Use the fixed system interpreter path, never a caller-controlled PATH lookup.
    // Dùng đường dẫn interpreter cố định của hệ thống, không dò PATH từ caller.
    let mut command = std::process::Command::new(windows_system_tool_path("cmd.exe")?);
    command
        .arg("/D")
        .arg("/S")
        .arg("/C")
        .arg(cmd_path)
        .args(args)
        .current_dir(cwd);
    Ok(command)
}
