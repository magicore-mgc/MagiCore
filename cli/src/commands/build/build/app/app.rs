use super::super::*;
use super::configuration::{read_app_build_config, resolve_app_language_from_config};
use super::multi::build_multi_app;

/// C9 — build app: single framework or all selected multi-platform targets.
/// Missing, unknown, or unsupported targets make the aggregate build fail.
#[cfg(feature = "app")]
pub(in crate::commands::build::implementation) async fn build_app(
    root: &Path,
    allow_output_warnings: bool,
    fetch: DependencyFetch,
) -> Result<()> {
    let config = read_app_build_config(root)?;
    let language = resolve_app_language_from_config(root, config.as_ref())?;

    // let-chain edition 2024 — gộp điều kiện theo clippy 1.98.
    if language == "multi"
        && let Some(v) = config.as_ref()
    {
        return build_multi_app(root, v, fetch);
    }

    // Load optimizer env for app runtime
    let runtime = detect_app_runtime(root);
    let optimizer_envs =
        crate::commands::optimizer::env_loader::load_optimizer_env(root, &runtime)?;
    let env: Vec<(String, String)> = optimizer_envs.into_iter().collect();
    let env_opt = if env.is_empty() { None } else { Some(env) };

    let gradle_args: Vec<&str> = gradle_build_args(fetch);
    let (tool, args): (&str, &[&str]) = match language {
        "kotlin" => ("gradle", gradle_args.as_slice()),
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
    post_build_app(root, allow_output_warnings)?;
    mgc_ui::success(&format!("App build completed ({language})"));
    Ok(())
}

/// Gradle build arguments for the selected dependency-fetch mode.
///
/// Gradle refuses to download a dependency without `--offline`, so online mode
/// omits it and offline mode keeps the fail-closed behavior.
/// (Tham số `gradle build` theo chế độ; online bỏ `--offline` để tải được
/// dependency còn thiếu, offline giữ fail-closed.)
fn gradle_build_args(fetch: DependencyFetch) -> Vec<&'static str> {
    if fetch.is_offline() {
        vec!["build", "--offline"]
    } else {
        vec!["build"]
    }
}

/// Describe the artefact a framework app build produced.
///
/// Flutter writes `build/`, and a web target nests `build/web/`; other targets
/// keep their output under `build/` too. The runner picks whichever exists so a
/// missing directory is a no-op rather than a failure.
/// Mô tả artefact mà build app framework tạo. Flutter ghi `build/`, target web
/// lồng `build/web/`; target khác cũng ở dưới `build/`. Runner chọn cái tồn tại
/// nên thư mục thiếu là no-op, không phải lỗi.
fn post_build_app(root: &Path, allow_output_warnings: bool) -> Result<()> {
    for candidate in ["build/web", "build"] {
        let dir = root.join(candidate);
        if dir.is_dir() {
            return crate::commands::build::post_build::run_for_core(
                root,
                "app",
                &dir,
                allow_output_warnings,
            );
        }
    }
    Ok(())
}
