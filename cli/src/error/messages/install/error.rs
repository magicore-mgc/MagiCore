//! Core install and language setup diagnostics.
//! Lỗi cài core và thiết lập ngôn ngữ.

use anyhow::Error;
use anyhow::anyhow;

/// Keep project dependency installs separate from persistent user CLI tools.
/// Tách dependency của project khỏi CLI tool bền vững theo user.
pub fn global_package_install_unsupported() -> Error {
    anyhow!(
        "`mgc add --global` is not supported: `mgc add` installs in the current project. Use `mgc tool install npm:<package>` for a persistent npm CLI or `mgc dlx <package>` for temporary execution; no project changes were made"
    )
}

/// A core without a preview implementation must not execute an install as a dry run.
/// Core chưa có preview phải từ chối trước khi chạy install với cờ dry-run.
pub fn install_dry_run_unsupported(core: &str) -> Error {
    anyhow!("dry-run is not supported for core '{core}'; no changes were made")
}

/// `mgc remove-ai <pkg> [pkg...]` — name packages to remove
pub fn remove_ai_usage() -> Error {
    anyhow!("mgc remove-ai <pkg> [pkg...] — name the packages to remove")
}

/// `mgc remove-app <pkg> [pkg...]` — name packages to remove
pub fn remove_app_usage() -> Error {
    anyhow!("mgc remove-app <pkg> [pkg...] — name the packages to remove")
}

/// `mgc add-app <pkg> [pkg...]` — name packages to add
pub fn add_app_usage() -> Error {
    anyhow!("mgc add-app <pkg> [pkg...] — name the packages to add")
}

pub fn app_not_available(reason: &str) -> Error {
    anyhow!("'app' {reason}")
}

pub fn app_project_not_detected() -> Error {
    anyhow!(
        "Cannot detect an app project here (missing mgc.toml [app] language / pubspec.yaml / build.gradle/.kts / Package.swift)."
    )
}

pub fn no_app_language(root: &std::path::Path) -> Error {
    anyhow!("No app language detected in {}", root.display())
}

pub fn app_tool_failed(cmd: &str, e: &dyn std::fmt::Display) -> Error {
    anyhow!("{cmd} failed: {e} — install the tool first and ensure it is in PATH")
}

/// verb không có CLI passthrough cho language
pub fn manifest_hint(verb: &str, lang: &str, file: &str) -> Error {
    anyhow!("'{verb}' for {lang:?} has no CLI passthrough — edit {file} then run `mgc install`.")
}

pub fn xcode_project_missing(root: &std::path::Path) -> Error {
    anyhow!(
        "no *.xcworkspace found/*.xcodeproj under {} — objC app missing an Xcode project",
        root.display()
    )
}

pub fn xcode_project_missing_short() -> Error {
    anyhow!("no *.xcworkspace found/*.xcodeproj — objC app missing an Xcode project")
}

pub fn objc_dev_needs_xcode(proj: &str) -> Error {
    anyhow!(
        "objC dev runs in Xcode — open {proj}, or set [app] dev_scheme = \"<scheme>\" in mgc.toml so `mgc dev` runs xcodebuild build on the simulator"
    )
}

pub fn ai_no_lockfile() -> Error {
    anyhow!(
        "no lock file (uv.lock or requirements.lock) — run `mgc add <pkg>` to create a lock first"
    )
}

pub fn ai_project_not_detected() -> Error {
    anyhow!(
        "Cannot detect an ai project here (missing mgc.toml [ai] framework / pyproject [tool.magicore] framework)."
    )
}

pub fn no_ai_framework(root: &std::path::Path) -> Error {
    anyhow!("No ai framework detected in {}", root.display())
}

pub fn python3_failed(e: &dyn std::fmt::Display) -> Error {
    anyhow!("python3 failed: {e} — install Python 3.11+ and ensure `python3` is in PATH")
}

/// Python launcher failed — Python launcher đã thất bại.
pub fn python_runtime_failed(launcher: &str, error: &dyn std::fmt::Display) -> Error {
    anyhow!("Python launcher '{launcher}' failed: {error}")
}

/// Native Python path cannot be represented in the child environment.
/// Không biểu diễn được native Python path trong môi trường tiến trình con.
pub fn python_path_not_unicode() -> Error {
    anyhow!(
        "native Python dependency path is not valid Unicode and cannot be passed through this command environment"
    )
}

/// Multi-platform install finished with skipped platforms (skip is not success).
pub fn app_multi_platforms_skipped(skipped: &[String]) -> Error {
    anyhow!(
        "install incomplete — skipped platform(s): {} (missing tool or runner; install them and re-run — a skip is never success)",
        skipped.join(", ")
    )
}
