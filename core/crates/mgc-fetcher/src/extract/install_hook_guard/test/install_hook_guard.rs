//! SEC-3 guard tests: classification, fail-closed default, escape hatch.
//! Test cổng SEC-3: phân loại, mặc định fail-closed, escape hatch.

use super::super::install_hook_guard::{HookKind, InstallHookGuard, classify};
use std::path::Path;
use std::sync::Mutex;

static ENV_LOCK: Mutex<()> = Mutex::new(());

fn with_hook_env<T>(value: Option<&str>, body: impl FnOnce() -> T) -> T {
    let _guard = ENV_LOCK
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let previous = std::env::var_os(super::super::ALLOW_INSTALL_HOOKS_ENV);
    match value {
        Some(raw) =>
        // SAFETY: test-only mutation of process-local state under `ENV_LOCK`.
        // AN TOÀN: chỉ test ghi state cục bộ process dưới `ENV_LOCK`.
        {
            #[allow(unsafe_code)]
            unsafe {
                std::env::set_var(super::super::ALLOW_INSTALL_HOOKS_ENV, raw)
            };
        }
        None =>
        // SAFETY: test-only mutation of process-local state under `ENV_LOCK`.
        // AN TOÀN: chỉ test ghi state cục bộ process dưới `ENV_LOCK`.
        {
            #[allow(unsafe_code)]
            unsafe {
                std::env::remove_var(super::super::ALLOW_INSTALL_HOOKS_ENV)
            };
        }
    }
    let result = body();
    match previous {
        Some(value) =>
        // SAFETY: restore under the same lock.
        // AN TOÀN: khôi phục dưới cùng lock.
        {
            #[allow(unsafe_code)]
            unsafe {
                std::env::set_var(super::super::ALLOW_INSTALL_HOOKS_ENV, value)
            };
        }
        None =>
        // SAFETY: restore under the same lock.
        // AN TOÀN: khôi phục dưới cùng lock.
        {
            #[allow(unsafe_code)]
            unsafe {
                std::env::remove_var(super::super::ALLOW_INSTALL_HOOKS_ENV)
            };
        }
    }
    result
}

#[test]
fn pth_entries_are_classified_regardless_of_case_and_depth() {
    for path in ["a.pth", "package/b.pth", "DIR/EVIL.PTH"] {
        assert!(classify(Path::new(path), None).is_some(), "{path}");
    }
}

#[test]
fn sitecustomize_nuget_scripts_and_privileged_modes_are_classified() {
    assert!(classify(Path::new("x/sitecustomize.py"), None).is_some());
    assert!(classify(Path::new("x/Usercustomize.py"), None).is_some());
    assert!(classify(Path::new("tools/INSTALL.ps1"), None).is_some());
    assert!(classify(Path::new("init.ps1"), None).is_some());
    assert!(classify(Path::new("bin/tool"), Some(0o4755)).is_some());
    assert!(classify(Path::new("bin/tool"), Some(0o2755)).is_some());
}

#[test]
fn symlink_mode_entries_are_classified_as_symlink_not_plain() {
    // A tar member whose mode declares S_IFLNK (0o120000) must be classified as
    // a symlink hook even though the plain-file bits are 0 — this is the guard
    // that stops a symlink entry from being written as a regular file.
    // Member tar có mode khai S_IFLNK (0o120000) phải bị phân loại là symlink dù
    // bit file thường bằng 0 — đây là cổng chặn entry symlink bị ghi như file thường.
    let classified = classify(Path::new("pkg/link"), Some(0o120644));
    assert_eq!(
        classified,
        Some(HookKind::SymlinkMode),
        "S_IFLNK mode must classify as SymlinkMode"
    );
}

#[test]
fn ordinary_entries_are_not_classified() {
    for path in [
        "package/package.json",
        "pkg/__init__.py",
        "docs/sitecustomize.md",
        "src/main.rs",
    ] {
        assert!(classify(Path::new(path), None).is_none(), "{path}");
    }
    assert!(classify(Path::new("bin/tool"), Some(0o755)).is_none());
}

#[test]
fn guard_denies_by_default_even_for_a_named_package() {
    let error = with_hook_env(None, || {
        InstallHookGuard::for_package("litellm@1.107.0").check(Path::new("evil.pth"), None)
    })
    .expect_err("guard must refuse without the env opt-in");
    assert!(format!("{error}").contains("evil.pth"));
    assert!(format!("{error}").contains("MAGICORE_ALLOW_INSTALL_HOOKS"));
}

#[test]
fn guard_allows_only_the_package_named_in_the_environment() {
    let allowed = with_hook_env(Some("litellm"), || {
        InstallHookGuard::for_package("litellm").check(Path::new("evil.pth"), None)
    });
    assert!(allowed.is_ok());
    let denied = with_hook_env(Some("some-other-package"), || {
        InstallHookGuard::for_package("litellm").check(Path::new("evil.pth"), None)
    });
    assert!(denied.is_err());
}

#[test]
fn deny_guard_never_consults_the_environment() {
    with_hook_env(Some("*"), || {
        assert!(
            InstallHookGuard::deny()
                .check(Path::new("evil.pth"), None)
                .is_err()
        );
    });
}
