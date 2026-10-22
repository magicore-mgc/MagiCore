//! Policy precedence and safe project configuration loading.
//! Thứ tự policy và đọc cấu hình project an toàn.

use std::path::Path;

use super::mode::LockPolicyMode;

/// Resolve policy; CI is always strict and cannot be downgraded by project or env config.
/// Ngoài CI: cờ CLI → env → `mgc.toml [lock]` → mặc định môi trường.
pub fn resolve_policy(flag: Option<&str>, project_root: Option<&Path>) -> LockPolicyMode {
    // CI is a release trust boundary; untrusted project config and ambient env
    // must not turn signature enforcement off.
    // CI là ranh giới tin cậy phát hành; config project/env không được hạ policy.
    let ci = std::env::var("CI")
        .map(|value| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes"
            )
        })
        .unwrap_or(false);
    if ci {
        return LockPolicyMode::Require;
    }

    if let Some(mode) = flag.and_then(LockPolicyMode::parse) {
        return mode;
    }
    if let Ok(raw) = std::env::var("MGC_LOCK_POLICY")
        && let Some(mode) = LockPolicyMode::parse(&raw)
    {
        return mode;
    }
    if let Some(root) = project_root
        && let Some(content) = read_project_config_no_follow(&root.join("mgc.toml"))
        && let Ok(value) = toml::from_str::<toml::Value>(&content)
        && let Some(policy) = value
            .get("lock")
            .and_then(|l| l.get("policy"))
            .and_then(|p| p.as_str())
        && let Some(mode) = LockPolicyMode::parse(policy)
    {
        return mode;
    }
    LockPolicyMode::environment_default()
}

/// Read project lock policy without following a symlink/reparse point.
/// Đọc lock policy project mà không theo symlink/reparse point.
fn read_project_config_no_follow(path: &Path) -> Option<String> {
    use std::io::Read;

    let metadata = std::fs::symlink_metadata(path).ok()?;
    if !metadata.file_type().is_file() {
        return None;
    }
    let mut options = std::fs::OpenOptions::new();
    options.read(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK);
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::OpenOptionsExt;
        const FILE_FLAG_OPEN_REPARSE_POINT: u32 = 0x0020_0000;
        options.custom_flags(FILE_FLAG_OPEN_REPARSE_POINT);
    }
    let file = options.open(path).ok()?;
    let opened = file.metadata().ok()?;
    if !opened.is_file() {
        return None;
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        const FILE_ATTRIBUTE_REPARSE_POINT: u32 = 0x0000_0400;
        if opened.file_attributes() & FILE_ATTRIBUTE_REPARSE_POINT != 0 {
            return None;
        }
    }
    const MAX_CONFIG_BYTES: u64 = 10 * 1024 * 1024;
    if opened.len() > MAX_CONFIG_BYTES {
        return None;
    }
    let mut content = String::new();
    file.take(MAX_CONFIG_BYTES + 1)
        .read_to_string(&mut content)
        .ok()?;
    (content.len() as u64 <= MAX_CONFIG_BYTES).then_some(content)
}
