use anyhow::Result;
use std::path::Path;
use std::time::Instant;

use super::{
    BuildEngine, EnginePreference, WebBuildTarget, build_native_engine, choose_engine,
    find_entry_point, find_native_engine_crate, resolve_web_build_target,
    run_framework_build_if_supported,
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
    engine_preference: EnginePreference,
    allow_output_warnings: bool,
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

    // Pick the engine from the framework's capability, not from whether
    // package.json happens to carry a build script.
    // Chọn engine theo năng lực framework, không theo việc package.json có tình
    // cờ có script build hay không.
    let framework = read_declared_framework(root);
    let has_script = has_framework_build_script(root);
    let (engine, engine_reason) =
        choose_engine(framework.as_deref(), has_script, engine_preference);
    info(&format!(
        "Engine: {} — {engine_reason}",
        match engine {
            BuildEngine::MagiCore => "MagiCore bundler",
            BuildEngine::Framework => "framework build program",
        }
    ));

    if engine == BuildEngine::Framework
        && run_framework_build_if_supported(root, compat, compat_runner)?
    {
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

        run_post_build(root, project_label(root), allow_output_warnings)?;
        return Ok(());
    }

    let entry = find_entry_point(root)?;
    info(&format!("Entry point: {}", entry.display()));

    let config = BundlerConfig {
        entry: entry.clone(),
        output_dir: root.join("dist"),
        minify: true,
        // A source map in a shipped bundle hands an attacker the original
        // sources and the build machine's paths. It is a debugging aid, so it
        // follows the debug profile instead of being unconditional: a release
        // build ships without one.
        // Source map trong bundle phát hành trao cho kẻ tấn công source gốc và
        // đường dẫn máy build. Nó là trợ giúp debug, nên đi theo profile debug
        // thay vì luôn bật: bản release phát hành không có nó.
        sourcemap: cfg!(debug_assertions),
        target: "es2020".to_string(),
        public_path: "/".to_string(),
        // A project without a browser entry point is a server build.
        // Project không có browser entry point là build phía server.
        backend: super::is_backend_project(root),
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

    run_post_build(root, project_label(root), allow_output_warnings)?;
    Ok(())
}

/// Name of the project for artefact metadata.
/// Tên project cho metadata artefact.
/// Framework id the project declares in `mgc.toml`.
/// Id framework mà project khai trong `mgc.toml`.
fn read_declared_framework(root: &Path) -> Option<String> {
    let raw = std::fs::read_to_string(root.join("mgc.toml")).ok()?;
    for line in raw.lines() {
        let trimmed = line.trim();
        let Some(list) = trimmed.strip_prefix("frameworks") else {
            continue;
        };
        let Some((_, rest)) = list.split_once('=') else {
            continue;
        };
        let inner = rest.trim().trim_start_matches('[').trim_end_matches(']');
        let first = inner.split(',').next()?.trim().trim_matches('"');
        if !first.is_empty() {
            return Some(first.to_string());
        }
    }
    None
}

/// Whether the project's `package.json` defines a build script.
/// Project có định nghĩa build script trong `package.json` không.
fn has_framework_build_script(root: &Path) -> bool {
    let Ok(raw) = std::fs::read_to_string(root.join("package.json")) else {
        return false;
    };
    let Ok(value) = serde_json::from_str::<serde_json::Value>(&raw) else {
        return false;
    };
    value
        .get("scripts")
        .and_then(|scripts| scripts.get("build"))
        .is_some_and(|build| build.is_string())
}

fn project_label(root: &Path) -> String {
    root.file_name()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_else(|| "app".to_string())
}

/// Describe, examine and sign whatever `dist/` now holds.
///
/// Delegates to the shared post-build runner so every core ships the same
/// assurances.
/// Giao cho runner sau build dùng chung để mọi core có cùng bảo đảm.
fn run_post_build(root: &Path, name: String, allow_warnings: bool) -> Result<()> {
    crate::commands::build::post_build::run_post_build(
        &root.join("dist"),
        crate::commands::build::post_build::PostBuildOptions {
            name: &name,
            version: "1.0.0",
            fail_on_findings: !allow_warnings,
            embed_identity: true,
        },
    )
}
