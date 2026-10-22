use anyhow::Result;
use std::path::Path;
use std::time::Instant;

use mgc_ui::info;

/// Build Rust with the complete optimizer environment — build Rust với toàn bộ env optimizer.
pub(crate) fn build_rust_with_env(
    root: &Path,
    optimizer_envs: std::collections::HashMap<String, String>,
) -> Result<()> {
    let start = Instant::now();
    info("Detected Rust project — running cargo build...");

    if let Some(rustflags) = optimizer_envs.get("RUSTFLAGS") {
        mgc_ui::info(&format!("Applying RUSTFLAGS: {rustflags}"));
    }

    let opts = mgc_exec::prelude::ExecOptions {
        cwd: Some(root.to_path_buf()),
        log_path: Some(root.join(".magicore").join("exec.log")),
        env: optimizer_envs.into_iter().collect(),
        clean_env: false, // Preserve existing env
        execution_scope: Some(mgc_exec::allowlist::ExecutionScope::BuildRunner),
        ..Default::default()
    };
    mgc_exec::prelude::run(
        "cargo",
        &[
            "build".to_string(),
            "--locked".to_string(),
            "--offline".to_string(),
        ],
        &opts,
    )?;

    let elapsed = start.elapsed();
    mgc_ui::success(&format!("Rust build completed in {:?}", elapsed));
    Ok(())
}

/// Build a library Rust project using only Cargo sources materialized by MGC.
/// Build Rust library bằng Cargo nhưng chỉ đọc source MGC đã materialize.
pub(crate) fn build_rust_with_managed_source(
    root: &Path,
    optimizer_envs: std::collections::HashMap<String, String>,
    source_root: &Path,
) -> Result<()> {
    let start = Instant::now();
    info("Detected Rust library — building from the MagiCore-managed offline source...");
    if let Some(rustflags) = optimizer_envs.get("RUSTFLAGS") {
        mgc_ui::info(&format!("Applying RUSTFLAGS: {rustflags}"));
    }
    let source_path = source_root.to_str().ok_or_else(|| {
        anyhow::Error::new(super::error::RustBuildError::ManagedSourcePathNotUtf8)
    })?;
    let directory_value = toml::Value::String(source_path.to_string()).to_string();
    let source_config = format!("source.mgc-managed.directory={directory_value}");
    let args = [
        "--config".to_string(),
        "source.crates-io.replace-with=\"mgc-managed\"".to_string(),
        "--config".to_string(),
        source_config,
        "build".to_string(),
        "--locked".to_string(),
        "--offline".to_string(),
    ];
    let opts = mgc_exec::prelude::ExecOptions {
        cwd: Some(root.to_path_buf()),
        log_path: Some(root.join(".magicore").join("exec.log")),
        env: optimizer_envs.into_iter().collect(),
        clean_env: false,
        execution_scope: Some(mgc_exec::allowlist::ExecutionScope::BuildRunner),
        ..Default::default()
    };
    mgc_exec::prelude::run("cargo", &args, &opts)?;
    mgc_ui::success(&format!(
        "Rust library build completed in {:?}",
        start.elapsed()
    ));
    Ok(())
}
