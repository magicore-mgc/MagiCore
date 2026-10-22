use anyhow::Result;
use std::path::{Path, PathBuf};
use std::time::Instant;

use super::super::tool_execution::run_allowlisted_tool_with_env;
use mgc_ui::info;

pub(crate) fn find_native_engine_crate(root: &Path) -> Option<PathBuf> {
    let candidates = [
        root.join("crates").join("engine"),
        root.join("apps")
            .join("frontend")
            .join("crates")
            .join("engine"),
    ];

    candidates
        .into_iter()
        .find(|path| path.join("Cargo.toml").exists())
}

pub(crate) fn build_native_engine(
    project_root: &Path,
    crate_dir: &Path,
    release: bool,
) -> Result<PathBuf> {
    let start = Instant::now();
    info(&format!(
        "Building native engine crate at {}...",
        crate_dir.display()
    ));

    // Load optimizer env from project root (not crate subdirectory)
    // Optimizer config is generated at project root: .mgc-optimizer/rust_cargo_profile.toml
    let runtime = crate::commands::optimizer::runtime_detect::DetectedRuntime::RustLib;
    let optimizer_envs =
        crate::commands::optimizer::env_loader::load_optimizer_env(project_root, &runtime)?;
    let env_opt = if optimizer_envs.is_empty() {
        None
    } else {
        Some(optimizer_envs.into_iter().collect())
    };

    let mut args = vec!["build", "--locked", "--offline"];
    if release {
        args.push("--release");
    }

    run_allowlisted_tool_with_env(crate_dir, "cargo", &args, env_opt)?;

    let binary_name = if cfg!(windows) {
        "mgc-web-engine.exe"
    } else {
        "mgc-web-engine"
    };
    let profile_dir = if release { "release" } else { "debug" };
    let binary = crate_dir.join("target").join(profile_dir).join(binary_name);
    if !binary.exists() {
        return Err(anyhow::Error::new(
            super::error::NativeEngineError::BinaryNotFound { path: binary },
        ));
    }

    info(&format!(
        "Native engine build completed in {:?}",
        start.elapsed()
    ));
    Ok(binary)
}
