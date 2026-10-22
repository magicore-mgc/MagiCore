//! Executable identity normalization and exact shell-probe recognition.
//! Chuẩn hóa identity executable và nhận diện đúng shell probe.

/// Recognize only the shell wrapper used for the exact read-only npm probe.
/// The resulting npm child is still checked against the authenticated shim.
/// Chỉ nhận shell wrapper của probe npm chỉ đọc chính xác; process npm con
/// vẫn phải qua xác thực shim riêng.
pub(in crate::run::implementation) fn is_npm_version_shell_probe(
    command: &[u8],
    command_line: &[u8],
) -> bool {
    let process = process_basename(&String::from_utf8_lossy(command));
    let Ok(line) = std::str::from_utf8(command_line) else {
        return false;
    };
    let Some((argv0, args)) = line.split_once(' ') else {
        return false;
    };
    match process.as_str() {
        // macOS may report the `/bin/sh` interpreter's image as `bash` while
        // argv[0] remains `/bin/sh`; accept the alias only for this exact probe.
        // (macOS có thể báo image của `/bin/sh` là `bash` dù argv[0] là `/bin/sh`;
        // chỉ nhận alias cho đúng probe này.)
        "sh" | "dash" | "bash" => {
            matches!(process_basename(argv0).as_str(), "sh" | "dash" | "bash")
                && args == "-c npm --version"
        }
        "busybox" => process_basename(argv0) == "busybox" && args == "sh -c npm --version",
        "cmd" => {
            if !process_basename(argv0).eq_ignore_ascii_case("cmd") {
                return false;
            }
            let mut parts = args.splitn(4, ' ');
            let (Some(disable_auto_run), Some(extensions), Some(command_flag), Some(script)) =
                (parts.next(), parts.next(), parts.next(), parts.next())
            else {
                return false;
            };
            disable_auto_run.eq_ignore_ascii_case("/d")
                && extensions.eq_ignore_ascii_case("/s")
                && command_flag.eq_ignore_ascii_case("/c")
                && script.trim_matches('"') == "npm --version"
        }
        _ => false,
    }
}

pub(in crate::run::implementation) fn process_basename(command: &str) -> String {
    let command = command.trim_matches(|ch| matches!(ch, '"' | '\'' | '`'));
    let base = command
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or(command)
        .to_ascii_lowercase();
    base.strip_suffix(".cmd")
        .or_else(|| base.strip_suffix(".exe"))
        .or_else(|| base.strip_suffix(".bat"))
        .or_else(|| base.strip_suffix(".ps1"))
        .unwrap_or(&base)
        .to_string()
}
