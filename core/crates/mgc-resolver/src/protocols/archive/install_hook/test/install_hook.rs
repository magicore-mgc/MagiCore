//! SEC-3 install-hook detector tests: classification, fail-closed gate, escape hatch.
//! Test detector hook SEC-3: phân loại, cổng fail-closed, escape hatch.

use super::super::install_hook::{
    ALLOW_INSTALL_HOOKS_ENV, FORBIDDEN_MODE_BITS, InstallHookKind, classify_entry,
    hooks_allowed_for, reject_install_hooks,
};
use std::sync::Mutex;

static ENV_LOCK: Mutex<()> = Mutex::new(());

fn with_hook_env<T>(value: Option<&str>, body: impl FnOnce() -> T) -> T {
    let _guard = ENV_LOCK
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let previous = std::env::var_os(ALLOW_INSTALL_HOOKS_ENV);
    match value {
        Some(raw) =>
        // SAFETY: test-only mutation of process-local state under `ENV_LOCK`.
        // AN TOÀN: chỉ test ghi state cục bộ process dưới `ENV_LOCK`.
        {
            #[allow(unsafe_code)]
            unsafe {
                std::env::set_var(ALLOW_INSTALL_HOOKS_ENV, raw)
            };
        }
        None =>
        // SAFETY: test-only mutation of process-local state under `ENV_LOCK`.
        // AN TOÀN: chỉ test ghi state cục bộ process dưới `ENV_LOCK`.
        {
            #[allow(unsafe_code)]
            unsafe {
                std::env::remove_var(ALLOW_INSTALL_HOOKS_ENV)
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
                std::env::set_var(ALLOW_INSTALL_HOOKS_ENV, value)
            };
        }
        None =>
        // SAFETY: restore under the same lock.
        // AN TOÀN: khôi phục dưới cùng lock.
        {
            #[allow(unsafe_code)]
            unsafe {
                std::env::remove_var(ALLOW_INSTALL_HOOKS_ENV)
            };
        }
    }
    result
}

#[test]
fn pth_file_is_refused_at_any_depth() {
    // The litellm 1.107.0 payload shape: a .pth placed in every wheel path.
    // (Hình dạng payload litellm 1.107.0: file .pth nằm ở mọi đường dẫn wheel.)
    for path in [
        "litellm-1.107.0.pth",
        "package/litellm.pth",
        "site-packages/zzz_evil.pth",
        "NESTED/DIR/evil.PTH",
    ] {
        let hook = classify_entry(path, None).expect("pth must be refused");
        assert_eq!(hook.kind, InstallHookKind::PythonPathConfig, "{path}");
    }
}

#[test]
fn sitecustomize_and_usercustomize_are_refused() {
    for path in [
        "sitecustomize.py",
        "package/usercustomize.py",
        "lib/python3.12/sitecustomize.py",
        "sitecustomize.PY",
    ] {
        let hook = classify_entry(path, None).expect("site hook must be refused");
        assert_eq!(hook.kind, InstallHookKind::PythonSiteCustomize, "{path}");
    }
}

#[test]
fn nuget_install_scripts_are_refused_case_insensitively() {
    for path in [
        "tools/install.ps1",
        "TOOLS/INSTALL.PS1",
        "content/tools/init.ps1",
        "install.ps1",
    ] {
        let hook = classify_entry(path, None).expect("nuget script must be refused");
        assert_eq!(hook.kind, InstallHookKind::NuGetInstallScript, "{path}");
    }
}

#[test]
fn setuid_and_setgid_modes_are_refused() {
    for mode in [
        FORBIDDEN_MODE_BITS,
        FORBIDDEN_MODE_BITS | 0o4000,
        0o4755,
        0o2750,
        0o6755,
    ] {
        let hook = classify_entry("bin/tool", Some(mode)).expect("privileged mode must be refused");
        assert_eq!(hook.kind, InstallHookKind::PrivilegedMode, "{mode:o}");
    }
}

#[test]
fn symlink_mode_entries_are_refused_as_symlink() {
    // A member whose mode word declares S_IFLNK must be refused as a symlink,
    // matching the fetcher guard, even when the tar typeflag is a regular file.
    // Entry khai S_IFLNK trong mode phải bị từ chối là symlink, khớp guard của
    // fetcher, dù typeflag tar là file thường.
    let hook = classify_entry("pkg/link", Some(0o120644)).expect("symlink mode must be refused");
    assert_eq!(hook.kind, InstallHookKind::SymlinkMode);
}

#[test]
fn ordinary_package_files_are_accepted() {
    for path in [
        "package/package.json",
        "package/index.js",
        "dist/main.py", // A regular Python module, not a site hook.
        "docs/sitecustomize.md",
        "src/path.rs",
        "tools/build.sh",
    ] {
        assert!(classify_entry(path, None).is_none(), "{path}");
    }
    assert!(classify_entry("package/bin", Some(0o755)).is_none());
}

#[test]
fn gate_refuses_the_first_hook_and_reports_its_path() {
    let entries = vec![
        ("package/package.json".to_string(), Some(0o644)),
        ("package/evil.pth".to_string(), None),
        ("package/sitecustomize.py".to_string(), None),
    ];
    let hook = reject_install_hooks(entries).expect_err("hook must be refused");
    assert_eq!(hook.path, "package/evil.pth");
    assert!(hook.to_string().contains("evil.pth"));
    assert!(hook.to_string().contains("Python .pth"));
}

#[test]
fn gate_accepts_a_clean_archive() {
    let entries = vec![
        ("package/package.json".to_string(), Some(0o644)),
        ("package/index.js".to_string(), Some(0o755)),
    ];
    assert!(reject_install_hooks(entries).is_ok());
}

#[test]
fn escape_hatch_is_deny_by_default() {
    let allowed = with_hook_env(None, || hooks_allowed_for("some-lib@1.0.0"));
    assert!(!allowed);
}

#[test]
fn escape_hatch_matches_name_and_name_at_version_case_insensitively() {
    let allowed = with_hook_env(Some(" Some-Lib@1.0.0 , other "), || {
        (
            hooks_allowed_for("some-lib@1.0.0"),
            hooks_allowed_for("some-lib@2.0.0"),
            hooks_allowed_for("other@9.9.9"),
            hooks_allowed_for("unlisted"),
        )
    });
    assert_eq!(allowed, (true, false, true, false));
}

#[test]
fn escape_hatch_ignores_empty_entries() {
    let allowed = with_hook_env(Some(" , ,"), || hooks_allowed_for("some-lib"));
    assert!(!allowed);
}
