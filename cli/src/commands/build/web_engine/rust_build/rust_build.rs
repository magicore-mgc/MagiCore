use anyhow::Result;
use std::path::Path;
use std::time::Instant;

use mgc_ui::info;

use crate::commands::build::implementation::DependencyFetch;

/// Build Rust with the complete optimizer environment — build Rust với toàn bộ env optimizer.
///
/// `fetch` decides whether Cargo may reach the network. The default is online:
/// an offline build that lacks a crate failed with Cargo's
/// `no matching package named …`, which reads like a broken project rather than
/// a fetch policy, and `mgc build --help` offered no way out.
/// (Chế độ `fetch` quyết định Cargo có được ra mạng không; mặc định online vì
/// build offline thiếu crate từng fail bằng message khó hiểu của Cargo.)
pub(crate) fn build_rust_with_env(
    root: &Path,
    optimizer_envs: std::collections::HashMap<String, String>,
    fetch: DependencyFetch,
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
    mgc_exec::prelude::run("cargo", &cargo_build_args(fetch, &[]), &opts)?;

    let elapsed = start.elapsed();
    mgc_ui::success(&format!("Rust build completed in {:?}", elapsed));
    Ok(())
}

/// Build a library Rust project using only Cargo sources materialized by MGC.
///
/// A managed source is a local directory replacement, so no network is
/// involved regardless of `fetch`; `--locked` keeps the lock authoritative.
/// (Build Rust thư viện bằng source MGC đã materialize; source là thư mục cục
/// bộ nên không cần mạng.)
pub(crate) fn build_rust_with_managed_source(
    root: &Path,
    optimizer_envs: std::collections::HashMap<String, String>,
    source_root: &Path,
    fetch: DependencyFetch,
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
    let base = [
        "--config".to_string(),
        "source.crates-io.replace-with=\"mgc-managed\"".to_string(),
        "--config".to_string(),
        source_config,
    ];
    let args = cargo_build_args(fetch, &base);
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

/// Assemble `cargo build` arguments for the selected dependency-fetch mode.
///
/// `--locked` is unconditional because MagiCore owns the lock; `--offline` is
/// added only when the caller asked for it, so the default build can fetch a
/// crate the store does not hold yet.
/// (Ghép tham số `cargo build`; `--locked` luôn có, `--offline` chỉ khi được
/// yêu cầu để build mặc định tải được crate chưa có.)
pub(crate) fn cargo_build_args(fetch: DependencyFetch, prefix: &[String]) -> Vec<String> {
    let mut args = prefix.to_vec();
    args.push("build".to_string());
    args.push("--locked".to_string());
    if fetch.is_offline() {
        args.push("--offline".to_string());
    }
    args
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
