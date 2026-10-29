#![allow(unsafe_code)] // `set_var`/`remove_var` are unsafe in edition 2024.
//! CI detection and policy resolution — phát hiện CI và chọn policy.
//!
//! The CI flag is a trust boundary: when it is true the install path forces
//! `Strict` signature verification. A wrong answer in either direction is a
//! security decision, so every plausible environment spelling is pinned here.
//! (Cờ CI là ranh giới tin cậy: khi đúng, install ép `Strict`. Trả lời sai theo
//! chiều nào cũng là quyết định bảo mật, nên mọi cách viết env khả dĩ đều được
//! ghim trong test.)

use super::super::policy::{is_ci_environment, resolve_effective_mode};

/// Serialises the cases that mutate process-wide environment.
///
/// `std::env` is one global map shared by every test thread in this binary, so
/// two cases running concurrently would clear each other's markers and produce
/// a failure that says nothing about the code.
/// (`std::env` là một map toàn cục dùng chung bởi mọi thread test, nên hai case
/// chạy song song sẽ xoá marker của nhau và báo lỗi không liên quan code.)
static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// Variables that leak the host's CI state into the assertion.
const CI_VARS: &[&str] = &[
    "CI",
    "GITHUB_ACTIONS",
    "GITLAB_CI",
    "CIRCLECI",
    "JENKINS_URL",
    "TRAVIS",
];

/// Clear every CI marker so each case starts from a known local state.
/// (Xoá mọi marker CI để mỗi case bắt đầu từ trạng thái local đã biết.)
fn clear_ci_vars() {
    for name in CI_VARS {
        // SAFETY(API): single-threaded test setup; `set_var` is unsafe in the
        // 2024 edition because other threads may read the environment.
        unsafe { std::env::remove_var(name) };
    }
}

fn with_var(name: &str, value: &str) {
    unsafe { std::env::set_var(name, value) };
}

/// `CI=false`, `CI=0`, `CI=no`, `CI=off` and `CI=` mean "not CI".
///
/// Treating a set-but-disabled flag as CI forced strict signature policy in a
/// developer shell, which looked like MagiCore refusing to install.
/// (`CI=false/0/no/off/""` nghĩa là KHÔNG phải CI. Coi nó là CI sẽ ép policy
/// strict trong shell dev, khiến MagiCore như từ chối cài.)
#[test]
fn a_disabled_ci_flag_is_not_ci() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    for value in ["0", "false", "FALSE", "False", "no", "off", "", "  "] {
        clear_ci_vars();
        with_var("CI", value);
        assert!(!is_ci_environment(), "CI={value:?} must not be read as CI");
    }
    clear_ci_vars();
}

/// Every enabled spelling is CI.
/// (Mọi cách viết bật đều là CI.)
#[test]
fn every_enabled_ci_spelling_is_ci() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    for value in ["1", "true", "TRUE", "yes", "on", "anything-else"] {
        clear_ci_vars();
        with_var("CI", value);
        assert!(is_ci_environment(), "CI={value:?} must be read as CI");
    }
    clear_ci_vars();
}

/// A provider marker counts by presence but not when empty.
///
/// Providers set these once; an empty value means the variable was neutralised,
/// which must not keep the install in strict mode.
/// (Marker provider tính theo sự tồn tại nhưng không tính khi rỗng.)
#[test]
fn provider_markers_count_unless_empty() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    for name in [
        "GITHUB_ACTIONS",
        "GITLAB_CI",
        "CIRCLECI",
        "JENKINS_URL",
        "TRAVIS",
    ] {
        clear_ci_vars();
        with_var(name, "true");
        assert!(is_ci_environment(), "{name}=true must be CI");

        clear_ci_vars();
        with_var(name, "");
        assert!(!is_ci_environment(), "{name}=<empty> must not be CI");
    }
    clear_ci_vars();
}

/// No marker at all is a local machine.
/// (Không marker nào là máy local.)
#[test]
fn no_markers_means_local() {
    let _guard = ENV_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    clear_ci_vars();
    assert!(!is_ci_environment());
}

/// CI always wins over a project or env policy that would relax verification.
/// (CI luôn thắng policy project/env muốn nới lỏng.)
#[test]
fn ci_forces_strict_over_any_relaxed_policy() {
    for (project, env) in [
        (Some("warn"), Some("warn")),
        (Some("off"), Some("audit")),
        (None, Some("off")),
        (Some("off"), None),
    ] {
        let mode = resolve_effective_mode(project, env, true);
        assert_eq!(
            mode,
            super::super::policy::PolicyMode::Strict,
            "CI must force strict for project={project:?} env={env:?}"
        );
    }
}

/// A local machine honours the explicit env policy.
/// (Máy local tôn trọng policy env rõ ràng.)
#[test]
fn a_local_machine_honours_the_explicit_policy() {
    let cases = [
        (Some("strict"), super::super::policy::PolicyMode::Strict),
        (Some("require"), super::super::policy::PolicyMode::Strict),
        (Some("warn"), super::super::policy::PolicyMode::Warn),
        (Some("audit"), super::super::policy::PolicyMode::Audit),
        (Some("off"), super::super::policy::PolicyMode::Audit),
    ];
    for (env, expected) in cases {
        assert_eq!(
            resolve_effective_mode(None, env, false),
            expected,
            "local env={env:?}"
        );
    }
}
