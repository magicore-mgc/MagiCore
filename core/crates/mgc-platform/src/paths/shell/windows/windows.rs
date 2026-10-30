//! Windows user PATH update through the OS user environment store.
//! Cập nhật PATH người dùng Windows qua kho môi trường của OS.

use std::path::Path;

#[cfg(windows)]
pub(in crate::paths::shell) const USER_PATH_SCRIPT: &str = r#"$ErrorActionPreference = 'Stop'
$bin = [IO.Path]::GetFullPath($env:MGC_TOOL_BIN_PATH)
$current = [Environment]::GetEnvironmentVariable('Path', 'User')
$parts = @($current -split ';' | Where-Object { -not [string]::IsNullOrWhiteSpace($_) })
$found = $false
foreach ($part in $parts) {
  try { if ([IO.Path]::GetFullPath([Environment]::ExpandEnvironmentVariables($part)).TrimEnd('\') -ieq $bin.TrimEnd('\')) { $found = $true } } catch { }
}
if (-not $found) { [Environment]::SetEnvironmentVariable('Path', (($bin + ';' + $current).Trim(';')), 'User') }
"#;

pub(in crate::paths::shell) fn ensure_windows_user_path(bin: &Path) -> anyhow::Result<()> {
    let output = std::process::Command::new("powershell.exe")
        .args([
            "-NoLogo",
            "-NoProfile",
            "-NonInteractive",
            "-Command",
            USER_PATH_SCRIPT,
        ])
        .env("MGC_TOOL_BIN_PATH", bin)
        .output()?;
    if !output.status.success() {
        anyhow::bail!(
            "could not add MagiCore's bin directory to the Windows user PATH: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    Ok(())
}
