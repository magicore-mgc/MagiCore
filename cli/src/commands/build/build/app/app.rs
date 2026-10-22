use super::super::*;
use super::configuration::{read_app_build_config, resolve_app_language_from_config};
use super::multi::build_multi_app;

/// C9 — build app: single framework or all selected multi-platform targets.
/// Missing, unknown, or unsupported targets make the aggregate build fail.
#[cfg(feature = "app")]
pub(in crate::commands::build::implementation) async fn build_app(root: &Path) -> Result<()> {
    let config = read_app_build_config(root)?;
    let language = resolve_app_language_from_config(root, config.as_ref())?;

    // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
    if language == "multi"
        && let Some(v) = config.as_ref()
    {
        return build_multi_app(root, v);
    }

    // Load optimizer env for app runtime
    let runtime = detect_app_runtime(root);
    let optimizer_envs =
        crate::commands::optimizer::env_loader::load_optimizer_env(root, &runtime)?;
    let env: Vec<(String, String)> = optimizer_envs.into_iter().collect();
    let env_opt = if env.is_empty() { None } else { Some(env) };

    let (tool, args): (&str, &[&str]) = match language {
        "kotlin" => ("gradle", &["build", "--offline"]),
        "swift" => (
            "swift",
            &["build", "--skip-update", "--disable-automatic-resolution"],
        ),
        // `flutter build web` is the universal CI target — no Android
        // SDK, no Xcode, runs on all three runners. `bundle` (the old
        // default) only produces asset dirs for mobile toolchains.
        // `flutter build web` là target CI phổ quát — không cần Android
        // SDK, không cần Xcode, chạy được cả ba runner. `bundle` (mặc
        // định cũ) chỉ sinh asset dir cho toolchain mobile.
        _ => ("flutter", &["build", "web", "--no-pub"]),
    };
    if tool_unavailable(tool) {
        return Err(crate::error::build_toolchain_missing(tool));
    }
    run_allowlisted_tool_with_env(root, tool, args, env_opt)?;
    mgc_ui::success(&format!("App build completed ({language})"));
    Ok(())
}
