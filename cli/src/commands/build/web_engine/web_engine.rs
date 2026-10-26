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
        choose_engine(framework.as_deref(), has_script, EnginePreference::Auto);
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

        run_post_build(root, project_label(root))?;
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

    run_post_build(root, project_label(root))?;
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
/// This runs whichever engine produced the output, because the question it
/// answers — "what exactly ships" — is the same for both. A blocking finding
/// fails the build: shipping bytes nobody examined is the failure mode this
/// exists to prevent.
/// Mô tả, xét và ký bất cứ thứ gì `dist/` đang giữ. Chạy bất kể engine nào tạo
/// output, vì câu hỏi nó trả lời — "chính xác cái gì được phát hành" — giống
/// nhau ở cả hai. Phát hiện chặn sẽ làm build fail: phát hành byte không ai xét
/// là kiểu hỏng mà bước này tồn tại để ngăn.
fn run_post_build(root: &Path, name: String) -> Result<()> {
    let dist = root.join("dist");
    if !dist.is_dir() {
        return Ok(());
    }

    let sbom_path = crate::commands::build::post_build::write_output_sbom(&dist, &name, "1.0.0")?;
    info(&format!("Output SBOM: {}", sbom_path.display()));

    let report = crate::commands::build::post_build::scan_output(&dist)?;
    info(&report.summary());
    if report.has_blocking() {
        anyhow::bail!(
            "build output failed the security scan; fix the findings above or pass --allow-output-warnings to ship anyway"
        );
    }

    // Embed the build's own identity so a deployed artefact can say what it is.
    // Nhúng danh tính của chính bản build để artefact đã deploy nói được nó là gì.
    let sbom_sha = crate::commands::build::post_build::sbom_sha256(&sbom_path)?;
    let touched = crate::commands::build::post_build::inject_build_identity(
        &dist,
        &crate::commands::build::post_build::BuildIdentity {
            name: name.clone(),
            version: "1.0.0".to_string(),
            sbom_sha256: sbom_sha,
        },
    )?;
    info(&format!("Build identity embedded in {touched} script(s)"));

    let keyring = mgc_platform::paths::mgc_data_dir().join("keyring.json");
    match crate::commands::build::post_build::sign_artifact(&sbom_path, &keyring)? {
        crate::commands::build::post_build::Signed::Signature(path) => {
            info(&format!("Signed: {}", path.display()));
        }
        crate::commands::build::post_build::Signed::Skipped(reason) => {
            info(&format!("Output not signed: {reason}"));
        }
    }
    Ok(())
}
