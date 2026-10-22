use super::super::*;
use super::WebGoBuildError;
use super::output_path::{ensure_go_build_output_dir, ensure_go_build_output_file};

pub(in crate::commands::build::implementation) const GO_BACKEND_LANGUAGE: &str = "go";

/// Compile a Go backend using MGC's verified module store and shared cache.
/// Build backend Go từ module store đã cài và cache tập trung của MGC.
#[cfg(feature = "web")]
pub(in crate::commands::build::implementation) fn build_web_go_backend(root: &Path) -> Result<()> {
    if tool_unavailable("go") {
        return Err(WebGoBuildError::ToolchainMissing { tool: "go" }.into());
    }
    crate::commands::freshness::check_any_freshness(root)?;
    crate::commands::freshness::check_web_go_build_freshness(root)?;

    let runtime = crate::commands::optimizer::runtime_detect::DetectedRuntime::GoLib;
    let optimizer_env = crate::commands::optimizer::env_loader::load_optimizer_env(root, &runtime)?;
    let global_paths = mgc_platform::paths::GlobalPaths::new()?;
    let go_modules = global_paths.store.join("go");
    let go_build_cache = global_paths.cache.join("go").join("build");
    std::fs::create_dir_all(&go_modules)?;
    std::fs::create_dir_all(&go_build_cache)?;

    let output_dir = ensure_go_build_output_dir(root)?;
    let output_name = if cfg!(windows) {
        "server.exe"
    } else {
        "server"
    };
    let output_path = output_dir.join(output_name);
    ensure_go_build_output_file(&output_path)?;

    let module_root = if root.join("cmd/server").is_dir() {
        "./cmd/server"
    } else {
        "."
    };
    let relative_output = format!(".magicore/build/go/{output_name}");
    let mut env: Vec<(String, String)> = optimizer_env.into_iter().collect();
    // Optimizer files cannot redirect dependency or compiled-artifact caches.
    // Optimizer không được chuyển hướng module store hoặc build cache của MGC.
    env.extend([
        ("GOMODCACHE".to_string(), go_modules.display().to_string()),
        ("GOCACHE".to_string(), go_build_cache.display().to_string()),
    ]);
    env.extend(go_offline_env());

    info(&format!(
        "Building Go Web backend: go build -mod=readonly -o {relative_output} {module_root}"
    ));
    run_allowlisted_tool_with_env(
        root,
        "go",
        &[
            "build",
            "-mod=readonly",
            "-o",
            &relative_output,
            module_root,
        ],
        Some(env),
    )
    .map_err(|source| WebGoBuildError::GoBuildFailed { source })?;
    mgc_ui::success(&format!(
        "Go Web backend build completed: {}",
        output_path.display()
    ));
    Ok(())
}
