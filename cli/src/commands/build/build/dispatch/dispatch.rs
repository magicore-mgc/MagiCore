use super::super::*;

pub async fn run(
    core: Option<&str>,
    target: Option<String>,
    compat_runtime: Option<&str>,
    compat_runner: Option<&str>,
    engine: Option<&str>,
    allow_output_warnings: bool,
) -> Result<()> {
    let build_engine_preference = crate::commands::build::engine_from_flag(engine)?;
    // Reject legacy rival-runtime flags before project discovery for every core.
    // (Từ chối cờ runtime đối thủ trước khi dò project, áp dụng đồng nhất mọi core.)
    let compat = crate::commands::compat::CompatMode::from_flag(compat_runtime)?;
    // Delegated runner lane (Spec rev4 §7/§9): qualified only, warned loudly.
    // (Lane runner ủy quyền (Spec rev4 §7/§9): chỉ qualified, cảnh báo lớn.)
    let compat_runner = crate::commands::compat::parse_compat_runner(compat_runner)?;
    ensure_native_build_mode(&compat)?;
    let root = find_root()?;

    if !mgc_ui::is_quiet() {
        mgc_ui::blank_line();
        println!("📦 {}", "MagiCore Build".bold().cyan());
    }
    info(&format!("Project root: {}", root.display()));

    if should_use_legacy_rust_build(&root)? {
        // Load optimizer env for standalone Rust projects
        // Tải env optimizer cho Rust project độc lập
        let runtime = crate::commands::optimizer::runtime_detect::DetectedRuntime::RustLib;
        let optimizer_envs =
            crate::commands::optimizer::env_loader::load_optimizer_env(&root, &runtime)?;
        return build_rust_with_env(&root, optimizer_envs);
    }

    let ctx = ProjectContext::load_with_core(core)?;
    info(&format!("Execution profile: {}", ctx.execution_summary()));
    // Spec rev4 §9/§11: compat contracts exist for web only; other cores
    // fail loudly instead of silently ignoring the flag.
    // (Spec rev4 §9/§11: mới web có compat contract; core khác fail rõ ràng
    // thay vì lặng lẽ bỏ qua flag.)
    if compat_runner.is_some() && ctx.adapter().name() != "web" {
        return Err(crate::error::compat_runner_unsupported_core(
            ctx.adapter().name(),
        ));
    }
    match ctx.adapter().name() {
        #[cfg(feature = "web")]
        "web" => {
            // Native Go web backends use the Go compiler instead of the JS bundler.
            // Backend Go dùng compiler Go, không bị rơi sang JS bundler.
            #[cfg(feature = "web")]
            if project_manifest_present(&root, "go.mod")?
                && !project_manifest_present(&root, "package.json")?
            {
                if target.is_some() {
                    return Err(crate::error::web_backend_flag_unsupported(
                        "--target",
                        GO_BACKEND_LANGUAGE,
                    ));
                }
                if compat_runner.is_some() {
                    return Err(crate::error::web_backend_flag_unsupported(
                        "--compat-runner",
                        GO_BACKEND_LANGUAGE,
                    ));
                }
                return build_web_go_backend(&root);
            }
            build_web(
                &root,
                ctx.execution(),
                target,
                &compat,
                compat_runner.as_deref(),
                build_engine_preference,
                allow_output_warnings,
            )
            .await
        }
        #[cfg(not(feature = "web"))]
        "web" => Err(crate::error::core_not_in_build("web")),
        #[cfg(feature = "app")]
        "app" => build_app(&root).await,
        #[cfg(not(feature = "app"))]
        "app" => Err(crate::error::core_not_in_build("app")),
        #[cfg(feature = "clo")]
        "cloud" => build_cloud(&root).await,
        #[cfg(not(feature = "clo"))]
        "cloud" => Err(crate::error::core_not_in_build("clo")),
        #[cfg(feature = "game")]
        "game" => build_game(&root).await,
        #[cfg(not(feature = "game"))]
        "game" => Err(crate::error::core_not_in_build("game")),
        #[cfg(feature = "iot")]
        "iot" => build_iot(&root).await,
        #[cfg(not(feature = "iot"))]
        "iot" => Err(crate::error::core_not_in_build("iot")),
        #[cfg(feature = "lib")]
        "lib" => build_lib(&root).await,
        #[cfg(not(feature = "lib"))]
        "lib" => Err(crate::error::core_not_in_build("lib")),
        #[cfg(feature = "hardware")]
        "hardware" => build_hardware(&root).await,
        #[cfg(not(feature = "hardware"))]
        "hardware" => Err(crate::error::core_not_in_build("hardware")),
        #[cfg(feature = "ai")]
        "ai" => build_ai(&root).await,
        #[cfg(not(feature = "ai"))]
        "ai" => Err(crate::error::core_not_in_build("ai")),
        other => bail!("'mgc build' not implemented for '{}' core yet", other),
    }
}
