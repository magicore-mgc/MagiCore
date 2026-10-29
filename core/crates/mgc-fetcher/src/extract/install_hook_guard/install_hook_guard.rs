//! SEC-3 gate for interpreter install-hook and privileged archive entries.
//! Cổng SEC-3 cho entry hook cài đặt interpreter và entry đặc quyền.
//!
//! `.pth` files are executed by Python's `site` module at every interpreter
//! start (the litellm 1.107.0 attack), `sitecustomize.py`/`usercustomize.py` are
//! imported automatically, and NuGet's `install.ps1` runs during install. None
//! of these need a package script to fire, so an install must refuse them
//! before any byte reaches the cache.
//! (File `.pth` được module `site` của Python chạy mỗi lần interpreter khởi
//! động (tấn công litellm 1.107.0), `sitecustomize.py`/`usercustomize.py` tự
//! import, `install.ps1` của NuGet chạy khi cài. Không cái nào cần package
//! script để kích hoạt, nên install phải từ chối trước khi byte nào vào cache.)

use super::super::archive_limits::{FORBIDDEN_MODE_BITS, NUGET_INSTALL_SCRIPT_BASENAMES, PYTHON_SITE_HOOK_MODULES};

/// Environment opt-in naming packages allowed to ship install-hook entries.
/// Biến môi trường nêu package được phép mang entry hook cài đặt.
pub const ALLOW_INSTALL_HOOKS_ENV: &str = "MAGICORE_ALLOW_INSTALL_HOOKS";
use super::super::{ExtractionError, ExtractionResult};
use std::path::Path;

/// How a refused entry was classified.
/// Cách entry bị từ chối được phân loại.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::extract) enum HookKind {
    PythonPathConfig,
    PythonSiteCustomize,
    NuGetInstallScript,
    PrivilegedMode,
}

impl HookKind {
    fn label(self) -> &'static str {
        match self {
            Self::PythonPathConfig => "Python .pth path-configuration hook",
            Self::PythonSiteCustomize => "Python sitecustomize/usercustomize hook",
            Self::NuGetInstallScript => "NuGet install.ps1 script",
            Self::PrivilegedMode => "setuid/setgid archive member",
        }
    }
}

/// Gate state for one extraction run.
/// Trạng thái cổng cho một lần giải nén.
#[derive(Debug, Clone)]
pub struct InstallHookGuard {
    package: Option<String>,
}

impl InstallHookGuard {
    /// Fail-closed: refuse every hook entry.
    /// Fail-closed: từ chối mọi entry hook.
    pub fn deny() -> Self {
        Self { package: None }
    }

    /// Allow hooks only for `package` AND only when the environment names it.
    /// Chỉ cho phép hook cho `package` VÀ chỉ khi environment nêu nó.
    pub fn for_package(package: impl Into<String>) -> Self {
        Self {
            package: Some(package.into()),
        }
    }

    /// Check one archive entry before it is staged.
    /// Kiểm một entry archive trước khi staging.
    pub fn check(&self, entry_path: &Path, mode: Option<u32>) -> ExtractionResult<()> {
        let Some(kind) = classify(entry_path, mode) else {
            return Ok(());
        };
        if let Some(package) = self.package.as_deref()
            && hooks_allowed_for(package)
        {
            // Loud, never silent: re-enabling a hook re-enables the attack.
            // (Cảnh báo rõ, không im lặng: bật lại hook là bật lại tấn công.)
            eprintln!(
                "WARNING: {ALLOW_INSTALL_HOOKS_ENV} allows {} for '{}': it can execute arbitrary code at interpreter/tool start",
                kind.label(),
                package
            );
            return Ok(());
        }
        Err(ExtractionError::InstallHookNotAllowed {
            path: entry_path.to_path_buf(),
            reason: kind.label(),
        })
    }
}

/// Classify an entry path (and optional POSIX mode word).
/// Phân loại path entry (và mode POSIX tùy chọn).
pub(in crate::extract) fn classify(entry_path: &Path, mode: Option<u32>) -> Option<HookKind> {
    if let Some(mode) = mode
        && mode & FORBIDDEN_MODE_BITS != 0
    {
        return Some(HookKind::PrivilegedMode);
    }
    let lossy = entry_path.to_string_lossy().replace('\\', "/").to_lowercase();
    let normalized = lossy.trim_start_matches("./");
    let basename = normalized.rsplit('/').next().unwrap_or("");
    if normalized.ends_with(".pth") {
        Some(HookKind::PythonPathConfig)
    } else if PYTHON_SITE_HOOK_MODULES.contains(&basename) {
        Some(HookKind::PythonSiteCustomize)
    } else if NUGET_INSTALL_SCRIPT_BASENAMES.contains(&basename) {
        Some(HookKind::NuGetInstallScript)
    } else {
        None
    }
}

/// Whether the environment names `package` (`name` or `name@version`).
/// Environment có nêu `package` (`name` hoặc `name@version`) không?
pub(in crate::extract) fn hooks_allowed_for(package: &str) -> bool {
    let raw = std::env::var(ALLOW_INSTALL_HOOKS_ENV).unwrap_or_default();
    hooks_allowed_in(&raw, package)
}

fn hooks_allowed_in(allowlist: &str, package: &str) -> bool {
    let wanted = package.trim().to_lowercase();
    let name = wanted.split('@').next().unwrap_or(&wanted).to_owned();
    allowlist
        .split(',')
        .map(|entry| entry.trim().to_lowercase())
        .filter(|entry| !entry.is_empty())
        .any(|entry| entry == wanted || entry == name)
}

#[cfg(test)]
#[path = "test/install_hook_guard.rs"]
mod tests;
