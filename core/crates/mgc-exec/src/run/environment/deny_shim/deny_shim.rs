//! Create platform-specific deny shims without changing directory lifecycle.
//! Tạo deny-shim theo nền tảng, không đổi vòng đời thư mục.

#[cfg(unix)]
/// Exact read-only npm shim body authenticated by the process guard.
/// Nội dung chính xác của shim npm chỉ đọc, được process guard xác thực.
pub(in crate::run) const NPM_VERSION_PROBE_SHIM_CONTENT: &str = concat!(
    "#!/bin/sh\n",
    "if [ \"$#\" -eq 1 ] && [ \"$1\" = \"--version\" ]; then\n",
    "  printf '%s\\n' '0.0.0'\n",
    "  exit 0\n",
    "fi\n",
    "printf '%s\\n' \"MagiCore blocked forbidden package manager: npm\" >&2\n",
    "exit 126\n",
);

#[cfg(unix)]
pub(in crate::run) fn write_blocker(dir: &std::path::Path, tool: &str) -> std::io::Result<()> {
    use std::io::Write;
    use std::os::unix::fs::PermissionsExt;

    let path = dir.join(tool);
    // Framework CLIs may ask npm for its version during a build. Answer that
    // read-only probe locally; every package-manager operation still fails.
    // CLI framework đôi khi hỏi phiên bản npm khi build. Trả lời probe đọc;
    // mọi thao tác package manager vẫn bị chặn.
    let content = if tool == "npm" {
        NPM_VERSION_PROBE_SHIM_CONTENT.to_string()
    } else {
        format!(
            "#!/bin/sh\nprintf '%s\\n' \"MagiCore blocked forbidden package manager: {tool}\" >&2\nexit 126\n"
        )
    };
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)?;
    file.write_all(content.as_bytes())?;
    drop(file);
    let mut permissions = std::fs::metadata(&path)?.permissions();
    permissions.set_mode(0o500);
    std::fs::set_permissions(&path, permissions)?;
    Ok(())
}

#[cfg(windows)]
pub(in crate::run) fn write_blocker(dir: &std::path::Path, tool: &str) -> std::io::Result<()> {
    use std::io::Write;

    // The shipped mgc executable is a native argv parser, so it can safely answer
    // exactly `npm --version` without routing untrusted arguments through CMD.
    // Executable mgc được phát hành phân tích argv native nên chỉ trả lời chính xác
    // `npm --version`, không đưa tham số không tin cậy qua CMD.
    if tool == "npm" {
        let executable = std::env::current_exe()?;
        let is_mgc_cli = executable
            .file_stem()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.eq_ignore_ascii_case("mgc"));
        if is_mgc_cli {
            let shim = dir.join("npm.exe");
            // Prefer a hard link so each guarded invocation does not duplicate the CLI image.
            // Ưu tiên hard link để mỗi lệnh được giám sát không phải nhân bản file CLI.
            if std::fs::hard_link(&executable, &shim).is_err() {
                std::fs::copy(&executable, &shim).map_err(|source| {
                    std::io::Error::new(
                        source.kind(),
                        format!(
                            "create native npm version-probe shim from '{}': {source}",
                            executable.display()
                        ),
                    )
                })?;
            }
            return Ok(());
        }
    }

    let path = dir.join(format!("{tool}.cmd"));
    // Keep Windows deny-only: CMD argument expansion is unsafe to interpolate into script syntax.
    // Windows luôn chặn npm: nội suy tham số CMD vào cú pháp script có thể tạo command injection.
    let content = format!(
        "@echo off\r\necho MagiCore blocked forbidden package manager: {tool} 1>&2\r\nexit /b 126\r\n"
    );
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
