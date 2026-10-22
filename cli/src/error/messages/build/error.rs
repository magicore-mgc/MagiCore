//! Build and bundler diagnostics.
//! Lỗi build và bundler.

use anyhow::Error;
use anyhow::anyhow;

/// python -m build failed
pub fn python_build_failed(e: &dyn std::fmt::Display) -> Error {
    anyhow!(
        "Python build backend failed: {e} — MagiCore does not install Python build tools; provide a supported build backend before retrying"
    )
}

/// go build ./... failed
pub fn go_build_failed(e: &dyn std::fmt::Display) -> Error {
    anyhow!("go build ./... failed: {e}")
}

/// dotnet build failed
pub fn dotnet_build_failed(e: &dyn std::fmt::Display) -> Error {
    anyhow!("dotnet build failed: {e} — install the .NET SDK first")
}

pub fn no_project_found_build() -> Error {
    anyhow!("No project found (no mgc.toml, package.json, or Cargo.toml)")
}

pub fn join_paths(err: &std::env::JoinPathsError) -> Error {
    anyhow!("{err}")
}

pub fn build_toolchain_missing(tool: &str) -> Error {
    anyhow!(
        "'{tool}' not found in PATH — install it first (mgc doctor check lists required toolchains)"
    )
}

pub fn java_build_backend_unavailable(root: &std::path::Path) -> Error {
    anyhow!(
        "Java build descriptor found in '{}', but MagiCore has no native Java build backend yet; refusing to route the project through TypeScript or invoke Maven/Gradle",
        root.display()
    )
}

/// Build has no supported artifact for the selected core — core này chưa có artifact build được hỗ trợ.
pub fn build_not_supported(core: &str, guidance: &str) -> Error {
    anyhow!("`mgc build` is not supported for core '{core}': {guidance}")
}

/// Build completed without producing any artifact — build không tạo được artifact nào.
pub fn build_no_artifact() -> Error {
    anyhow!(
        "build produced no artifact; fix the project configuration or install the required toolchain"
    )
}

/// Refuse to infer an App build target from a malformed `mgc.toml`.
pub fn app_build_config_invalid(path: &std::path::Path, error: &dyn std::fmt::Display) -> Error {
    anyhow!("invalid App build config '{}': {error}", path.display())
}

/// Report App config I/O errors instead of treating unreadable config as absent.
pub fn app_build_config_read_failed(
    path: &std::path::Path,
    error: &dyn std::fmt::Display,
) -> Error {
    anyhow!("cannot read App build config '{}': {error}", path.display())
}

/// Explicit App language must have a build implementation.
pub fn app_build_language_invalid(language: &str) -> Error {
    anyhow!("unsupported or invalid App build language '{language}'")
}

/// Multiple App build runtimes require explicit project configuration.
/// Nhiều runtime App cần được khai báo tường minh trong cấu hình dự án.
pub fn app_build_language_ambiguous(languages: &[&str]) -> Error {
    anyhow!(
        "App build runtime is ambiguous ({}); set `app.language` explicitly",
        languages.join(", ")
    )
}

/// App multi-platform configuration must be explicit and well-formed.
/// Cấu hình đa nền tảng của App phải tường minh và hợp lệ.
pub fn app_build_platforms_invalid(reason: &str) -> Error {
    anyhow!("invalid App build platforms configuration: {reason}")
}

/// A multi-platform App build is incomplete when any selected target was skipped.
pub fn build_multi_platforms_incomplete(skipped: &[String]) -> Error {
    anyhow!(
        "multi-platform build incomplete; selected target(s) not built: {}",
        skipped.join(", ")
    )
}

pub fn esbuild_failed(msgs: &[String]) -> Error {
    anyhow!("esbuild failed: {}", msgs.join("\n"))
}

pub fn link_node_modules_failed(
    source: &std::path::Path,
    target: &std::path::Path,
    err: &dyn std::fmt::Display,
) -> Error {
    anyhow!(
        "failed to link node_modules for build from {} to {}: {}",
        source.display(),
        target.display(),
        err
    )
}

/// A native build output path contains a symlink or a non-directory component.
/// Đường dẫn output build native chứa symlink hoặc thành phần không phải thư mục.
pub fn build_output_path_unsafe(path: &std::path::Path, reason: &str) -> Error {
    anyhow!(
        "refusing native build output '{}': {reason}",
        path.display()
    )
}

/// Go build cannot prove dependency ownership from the current manifest shape.
/// Go build không chứng minh được dependency ownership từ dạng manifest hiện tại.
pub fn go_build_freshness_unverified(reason: &str) -> Error {
    anyhow!(
        "cannot verify Go dependency ownership before build: {reason}; remove unsupported go.mod replace directives or use a qualified native install flow"
    )
}
