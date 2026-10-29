//! Reject interpreter install-hook entries and privileged archive members.
//! Từ chối entry là hook cài đặt của interpreter và thành viên archive đặc quyền.
//!
//! SEC-3: the litellm 1.107.0 incident shipped a `.pth` file that Python's
//! `site` module imports at *every* interpreter start, so installing "just a
//! library" executed attacker code with no package script in sight. The same
//! class exists for `sitecustomize.py`/`usercustomize.py` and for NuGet's
//! `tools/install.ps1` (run by the NuGet client at install time). This module
//! is the single fail-closed detector shared by every materialization path.
//! (SEC-3: sự cố litellm 1.107.0 kèm file `.pth` mà module `site` của Python
//! import ở MỌI lần khởi động interpreter, nên cài "chỉ một thư viện" đã chạy
//! code tấn công dù không có package script nào. Cùng nhóm này có
//! `sitecustomize.py`/`usercustomize.py` và `tools/install.ps1` của NuGet
//! (NuGet client chạy khi cài). Module này là detector fail-closed duy nhất
//! dùng chung cho mọi đường materialize.)

/// Environment opt-in that allows install-hook entries for explicitly named
/// package (matched case-insensitively against `name@version` or bare `name`).
/// Each allowed package is announced loudly because it re-enables the exact
/// attack this detector blocks.
/// Biến môi trường opt-in cho phép entry hook cho các package được nêu tường
/// minh (khớp không phân biệt hoa thường với `name@version` hoặc `name`). Mỗi
/// package được cho phép đều cảnh báo rõ vì nó mở lại cuộc tấn công bị chặn.)
pub const ALLOW_INSTALL_HOOKS_ENV: &str = "MAGICORE_ALLOW_INSTALL_HOOKS";

/// Why an entry was refused.
/// Lý do entry bị từ chối.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallHookKind {
    /// Python path-configuration file executed by `site` at interpreter start.
    /// File cấu hình path Python được `site` chạy khi interpreter khởi động.
    PythonPathConfig,
    /// Python module auto-imported by `site` at interpreter start.
    /// Module Python được `site` tự import khi interpreter khởi động.
    PythonSiteCustomize,
    /// NuGet PowerShell install script executed by the NuGet client.
    /// Script PowerShell cài đặt NuGet do NuGet client chạy.
    NuGetInstallScript,
    /// setuid/setgid archive member — never legitimate in a package.
    /// Thành viên archive setuid/setgid — không bao giờ hợp lệ trong package.
    PrivilegedMode,
}

impl InstallHookKind {
    /// Human-readable label used in error messages (English console output).
    /// Nhãn đọc được dùng trong thông báo lỗi (console tiếng Anh).
    pub fn label(self) -> &'static str {
        match self {
            Self::PythonPathConfig => "Python .pth path-configuration hook",
            Self::PythonSiteCustomize => "Python sitecustomize/usercustomize hook",
            Self::NuGetInstallScript => "NuGet install.ps1 script",
            Self::PrivilegedMode => "setuid/setgid archive member",
        }
    }
}

/// A refused entry and the reason.
/// Entry bị từ chối kèm lý do.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallHook {
    pub path: String,
    pub kind: InstallHookKind,
}

impl std::fmt::Display for InstallHook {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "refusing archive entry '{}': {}",
            self.path,
            self.kind.label()
        )
    }
}

impl std::error::Error for InstallHook {}

impl From<InstallHook> for crate::protocols::archive::ArchiveError {
    fn from(hook: InstallHook) -> Self {
        Self::Integrity(hook.to_string())
    }
}

/// Files that carry executable install-hook semantics, whatever the ecosystem.
/// Các file mang ngữ nghĩa hook cài đặt, bất kể ecosystem.
const PYTHON_SITE_HOOK_MODULES: &[&str] = &["sitecustomize.py", "usercustomize.py"];

/// NuGet install-script basenames. NuGet runs `install.ps1`/`init.ps1` from
/// `tools/` in the old packages.config model; any such basename in any
/// directory is refused because a repacked nupkg can hide it elsewhere.
/// Basename script cài đặt NuGet. NuGet chạy `install.ps1`/`init.ps1` trong
/// `tools/` theo model packages.config cũ; mọi basename đó ở bất kỳ thư mục nào
/// đều bị từ chối vì nupkg đóng lại có thể giấu ở chỗ khác.
const NUGET_INSTALL_SCRIPT_BASENAMES: &[&str] = &["install.ps1", "init.ps1"];

/// POSIX mode bits that must never be set by an extracted archive entry.
/// Bit mode POSIX mà entry archive giải nén không bao giờ được bật.
pub const FORBIDDEN_MODE_BITS: u32 = 0o6000;

/// Classify an archive entry path, returning its hook kind when refused.
/// `mode` is the POSIX permission word when the archive carries one.
/// Phân loại path entry archive, trả loại hook khi bị từ chối. `mode` là mode
/// quyền POSIX khi archive có mang.
pub fn classify_entry(entry_path: &str, mode: Option<u32>) -> Option<InstallHook> {
    if let Some(mode) = mode
        && mode & FORBIDDEN_MODE_BITS != 0
    {
        return Some(InstallHook {
            path: entry_path.to_owned(),
            kind: InstallHookKind::PrivilegedMode,
        });
    }
    let normalized = normalize_archive_name(entry_path);
    let basename = normalized.rsplit('/').next().unwrap_or("");
    let kind = if normalized.ends_with(".pth") {
        InstallHookKind::PythonPathConfig
    } else if PYTHON_SITE_HOOK_MODULES.contains(&basename) {
        InstallHookKind::PythonSiteCustomize
    } else if NUGET_INSTALL_SCRIPT_BASENAMES.contains(&basename) {
        InstallHookKind::NuGetInstallScript
    } else {
        return None;
    };
    Some(InstallHook {
        path: entry_path.to_owned(),
        kind,
    })
}

/// Normalize an archive name for classification: strip a leading `./`, unify
/// separators, lower-case. Case folding matters because Windows and NuGet
/// paths are case-insensitive (`TOOLS/INSTALL.PS1` runs the same script).
/// Chuẩn hóa tên archive để phân loại: bỏ `./` đầu, đồng nhất dấu phân cách, viết
/// thường. Gập hoa thường quan trọng vì path Windows/NuGet không phân biệt
/// hoa thường (`TOOLS/INSTALL.PS1` vẫn chạy cùng script).
fn normalize_archive_name(entry_path: &str) -> String {
    let replaced = entry_path.replace('\\', "/");
    let trimmed = replaced.trim_start_matches("./");
    trimmed.to_lowercase()
}

/// Fail-closed gate: return the first refused entry, if any.
/// Cổng fail-closed: trả entry bị từ chối đầu tiên, nếu có.
pub fn reject_install_hooks(
    entries: impl IntoIterator<Item = (String, Option<u32>)>,
) -> Result<(), InstallHook> {
    for (path, mode) in entries {
        if let Some(hook) = classify_entry(&path, mode) {
            return Err(hook);
        }
    }
    Ok(())
}

/// Whether the current environment explicitly allows hooks for `package`.
/// Package identity is `name` optionally followed by `@version`. An allowlist
/// entry without a version allows every version of that name; an entry with a
/// version allows exactly that name@version.
/// (Environment có cho phép hook cho `package` tường minh không? Định danh là
/// `name` có thể kèm `@version`. Entry allowlist không có version cho phép mọi
/// version của tên đó; entry có version chỉ cho phép đúng name@version.)
pub fn hooks_allowed_for(package: &str) -> bool {
    let raw = std::env::var(ALLOW_INSTALL_HOOKS_ENV).unwrap_or_default();
    hooks_allowed_in(&raw, package)
}

/// Pure allowlist match, split out so the rule is testable without the env.
/// So khớp allowlist thuần, tách ra để test được quy tắc không cần env.
fn hooks_allowed_in(allowlist: &str, package: &str) -> bool {
    let wanted = package.trim().to_lowercase();
    let name = wanted.split('@').next().unwrap_or(&wanted).to_owned();
    allowlist
        .split(',')
        .map(|entry| entry.trim().to_lowercase())
        .filter(|entry| !entry.is_empty())
        .any(|entry| entry == wanted || entry == name)
}

/// Loud, once-per-process announcement that an install-hook escape hatch was
/// used for a specific package. Never silent.
/// Cảnh báo rõ một lần mỗi process rằng escape hatch hook đã dùng cho một
/// package cụ thể. Không bao giờ im lặng.
pub fn warn_hook_escape_hatch(package: &str, hook: &InstallHook) {
    eprintln!(
        "WARNING: {ALLOW_INSTALL_HOOKS_ENV} allows {} for package '{package}': installing it can execute arbitrary code at interpreter/tool start",
        hook.kind.label()
    );
}

/// Install-hook gate for one extraction run.
/// Cổng hook cài đặt cho một lần giải nén.
#[derive(Debug, Clone)]
pub struct InstallHookPolicy {
    package: Option<String>,
}

impl InstallHookPolicy {
    /// Fail-closed default: refuse every hook entry.
    /// Mặc định fail-closed: từ chối mọi entry hook.
    pub fn deny() -> Self {
        Self { package: None }
    }

    /// Allow hooks only for `package`, and only when the environment names it
    /// in `MAGICORE_ALLOW_INSTALL_HOOKS`. Naming the package is not enough on
    /// its own: the operator opt-in is still required.
    /// (Cho phép hook chỉ cho `package`, và chỉ khi environment nêu nó trong
    /// `MAGICORE_ALLOW_INSTALL_HOOKS`. Nêu tên package chưa đủ: vẫn cần opt-in
    /// của operator.)
    pub fn for_package(package: impl Into<String>) -> Self {
        Self {
            package: Some(package.into()),
        }
    }

    /// Check one entry; returns the refusal, or logs the escape-hatch warning
    /// when this package's hooks were explicitly enabled.
    /// Kiểm một entry; trả từ chối, hoặc ghi cảnh báo escape hatch khi hook của
    /// package này đã được bật tường minh.
    pub fn check(&self, entry_path: &str, mode: Option<u32>) -> Result<(), InstallHook> {
        let Some(hook) = classify_entry(entry_path, mode) else {
            return Ok(());
        };
        let Some(package) = self.package.as_deref() else {
            return Err(hook);
        };
        if hooks_allowed_for(package) {
            warn_hook_escape_hatch(package, &hook);
            return Ok(());
        }
        Err(InstallHook {
            path: hook.path,
            kind: hook.kind,
        })
    }
}

#[cfg(test)]
#[path = "test/install_hook.rs"]
mod tests;
