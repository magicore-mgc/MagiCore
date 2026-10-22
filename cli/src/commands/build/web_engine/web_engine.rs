use anyhow::Result;
use std::path::Path;
use std::time::Instant;

use super::{
    WebBuildTarget, build_native_engine, find_entry_point, find_native_engine_crate,
    resolve_web_build_target, run_framework_build_if_supported,
};
use crate::bundler::{Bundler, BundlerConfig};
use mgc_config::project::ProjectExecutionConfig;
use mgc_ui::info;

pub(crate) async fn build_web(
    root: &Path,
    execution: &ProjectExecutionConfig,
    target: Option<String>,
    compat: &crate::commands::compat::CompatMode,
    compat_runner: Option<&str>,
) -> Result<()> {
    let start_time = Instant::now();

    let resolved_target = resolve_web_build_target(execution, target.as_deref());
    info(&format!("Resolved build lane: {}", resolved_target.label()));

    match resolved_target {
        WebBuildTarget::CompatibilityShell => {
            info("Engine Web: Running compatibility-shell bundler...");
        }
        WebBuildTarget::NativeReady => {
            info(
                "Engine Web: Running compatibility-shell build with native-ready bridge metadata...",
            );
            info(
                "Native-ready lane keeps framework compatibility while preparing Rust/native execution surfaces.",
            );
        }
        WebBuildTarget::CompiledExecutable => {
            info("Engine Web: Compiled executable lane selected.");
            info(
                "MagiCore will build web assets first, then compile the Rust-native engine executable.",
            );
        }
    }

    if run_framework_build_if_supported(root, compat, compat_runner)? {
        let elapsed = start_time.elapsed();
        mgc_ui::blank_line();
        mgc_ui::success(&format!("Framework build completed in {:?}", elapsed));

        if matches!(
            resolved_target,
            WebBuildTarget::NativeReady | WebBuildTarget::CompiledExecutable
        ) {
            if let Some(engine_crate) = find_native_engine_crate(root) {
                let binary = build_native_engine(
                    root, // project root for optimizer config
                    &engine_crate,
                    matches!(resolved_target, WebBuildTarget::CompiledExecutable),
                )?;
                mgc_ui::success(&format!("Native engine binary ready: {}", binary.display()));
            } else {
                info(
                    "No native engine crate detected for this project; compatibility artifact is still ready.",
                );
            }
        }

        return Ok(());
    }

    let entry = find_entry_point(root)?;
    info(&format!("Entry point: {}", entry.display()));

    let config = BundlerConfig {
        entry: entry.clone(),
        output_dir: root.join("dist"),
        minify: true,
        sourcemap: true,
        target: "es2020".to_string(),
        public_path: "/".to_string(),
    };

    let bundler = Bundler::new(config.clone());
    let result = bundler.bundle().await?;

    let elapsed = start_time.elapsed();
    mgc_ui::blank_line();
    mgc_ui::success(&format!(
        "Bundle created: {:.2} KB in {:?}",
        result.size as f64 / 1024.0,
        elapsed
    ));

    info("Processing assets...");
    crate::bundler::process_assets(&config).await?;

    if matches!(
        resolved_target,
        WebBuildTarget::NativeReady | WebBuildTarget::CompiledExecutable
    ) {
        if let Some(engine_crate) = find_native_engine_crate(root) {
            let binary = build_native_engine(
                root, // project root for optimizer config
                &engine_crate,
                matches!(resolved_target, WebBuildTarget::CompiledExecutable),
            )?;
            mgc_ui::success(&format!("Native engine binary ready: {}", binary.display()));
        } else {
            info(
                "No native engine crate detected for this project; compatibility artifact is still ready.",
            );
        }
    }

    Ok(())
}
