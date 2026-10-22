//! Web package, install, and dev diagnostics.
//! Lỗi package, install và dev của Web.

use anyhow::Error;
use anyhow::anyhow;

/// web project missing package.json — cannot run tests
pub fn web_missing_package_json() -> Error {
    anyhow!("web project is missing package.json — cannot run tests")
}

pub fn topo_order_failed(e: &dyn std::fmt::Display) -> Error {
    anyhow!("monorepo topo order failed: {e}")
}

pub fn install_slot_failed(e: &dyn std::fmt::Display) -> Error {
    anyhow!("failed to acquire install slot: {e}")
}

pub fn install_task_failed(e: &dyn std::fmt::Display) -> Error {
    anyhow!("monorepo install task failed: {e}")
}

pub fn no_framework_specified() -> Error {
    anyhow!(
        "No framework specified. Use a flag like --react, --next, --vue, or pass a framework name as the first argument."
    )
}

pub fn pm_not_supported(pm: &str) -> Error {
    anyhow!(
        "--pm {pm} is not supported by core-web. MagiCore web uses the native mgc installer, not npm/pnpm/yarn/bun."
    )
}

pub fn ts_js_exclusive() -> Error {
    anyhow!("--ts and --js are mutually exclusive")
}

pub fn multiple_frameworks() -> Error {
    anyhow!(
        "Multiple frontend frameworks specified. Choose only one (--react, --next, --vue, etc.)."
    )
}

pub fn pinia_requires_vue() -> Error {
    anyhow!("--pinia requires --vue or --nuxt")
}

pub fn no_primary_package(fw: &str) -> Error {
    anyhow!("framework '{fw}' does not declare a primary package")
}

pub fn network_error_fetching(package: &str, e: &dyn std::fmt::Display) -> Error {
    anyhow!("network error fetching '{package}': {e}")
}

pub fn npm_registry_status(package: &str, status: &reqwest::StatusCode) -> Error {
    anyhow!("npm registry returned {status} for '{package}'")
}

pub fn bad_npm_response(package: &str, e: &dyn std::fmt::Display) -> Error {
    anyhow!("bad npm response for '{package}': {e}")
}

pub fn no_version_field(package: &str) -> Error {
    anyhow!("no version field for '{package}'")
}

pub fn web_no_dev_target(root: &std::path::Path, hint: &str) -> Error {
    anyhow!(
        "No runnable dev target found in '{}'. Run '{}' first.",
        root.display(),
        hint
    )
}

pub fn web_workspace_dep_missing(
    dep: &str,
    target: &std::path::Path,
    root: &std::path::Path,
) -> Error {
    anyhow!(
        "workspace dependency '{dep}' referenced by '{}' was not found under '{}'",
        target.display(),
        root.display()
    )
}

pub fn web_empty_dev_script(root: &std::path::Path) -> Error {
    anyhow!(
        "Empty dev script in '{}'",
        root.join("package.json").display()
    )
}

pub fn web_unsupported_dev_script(script: &str, root: &std::path::Path) -> Error {
    anyhow!(
        "Unsupported dev script '{script}' in '{}'",
        root.join("package.json").display()
    )
}

pub fn web_forbidden_pm(script: &str, manifest: &std::path::Path, pm: &str) -> Error {
    anyhow!(
        "Unsupported script '{script}' in '{}': it delegates to '{pm}'. Core-web must execute natively through MagiCore or framework-local binaries, not through another package manager.",
        manifest.display()
    )
}

pub fn web_no_dev_entrypoint(root: &std::path::Path) -> Error {
    anyhow!(
        "No 'dev' script found in '{}' and no supported native dev entrypoint was detected",
        root.display()
    )
}

/// A dev process group received SIGINT/SIGTERM.
/// Nhóm dev nhận SIGINT/SIGTERM và đang được dọn dẹp.
pub fn web_dev_interrupted() -> Error {
    anyhow!("dev server group interrupted by SIGINT/SIGTERM")
}

/// A sibling dev target ended and this target is being cleaned up.
/// Target dev cùng nhóm kết thúc nên target này đang được dọn dẹp.
pub fn web_dev_target_cancelled() -> Error {
    anyhow!("dev target cancelled because a sibling target stopped")
}

pub fn web_no_install_flow(root: &std::path::Path) -> Error {
    anyhow!(
        "No supported native install flow found in '{}'",
        root.display()
    )
}

pub fn web_missing_executable(bin: &str, hint: &str, root: &std::path::Path) -> Error {
    anyhow!(
        "Missing local executable '{bin}'. Run '{hint}' in '{}'.",
        root.display()
    )
}

pub fn web_generator_output_invalid(path: &std::path::Path) -> Error {
    anyhow!(
        "online framework generator did not produce a valid project at '{}'",
        path.display()
    )
}

pub fn web_generator_output_unsafe(path: &std::path::Path) -> Error {
    anyhow!(
        "online framework generator output '{}' contains a linked or special path",
        path.display()
    )
}

/// A web-pipeline-only flag was passed for a non-JS backend lane — failed loudly
/// instead of silently dropping the flag.
/// (Flag chỉ-dành-web pipeline dùng cho backend non-JS — fail rõ.)
pub fn web_backend_flag_unsupported(flag: &str, language: &str) -> Error {
    anyhow!(
        "`{flag}` applies to the JavaScript pipeline only and is not supported for the {language} backend lane — omit the flag"
    )
}

/// A selected framework must match a native container manifest profile.
/// Framework phải khớp manifest container native được hỗ trợ.
/// A scaffold framework spec failed canonical parsing.
/// Framework spec scaffold không qua được bước phân tích tên chuẩn.
pub fn invalid_framework_spec(core: &str, framework: &str, detail: &str) -> Error {
    anyhow!("invalid {core} framework specification '{framework}': {detail}")
}
