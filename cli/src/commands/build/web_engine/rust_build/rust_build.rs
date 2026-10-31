use anyhow::Result;
use std::path::Path;
use std::time::Instant;

use mgc_ui::info;

use crate::commands::build::implementation::DependencyFetch;

/// Name Cargo uses for the MagiCore-managed directory source.
/// Tên Cargo dùng cho directory source do MagiCore quản lý.
pub(crate) const MANAGED_SOURCE_NAME: &str = "mgc-managed";

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

/// Build a Rust library project against the MagiCore-managed crate source.
///
/// The managed store is a fallback, never the only source. Offline replaces
/// crates-io with the directory source and fails closed on a crate it lacks;
/// online only *adds* the directory as a named source and leaves crates-io
/// primary, because a directory source cannot fetch and replacing crates-io
/// made every online build unable to resolve a crate the store did not hold.
/// `--locked` stays in both modes.
/// (Store managed là fallback, không phải nguồn duy nhất. Offline thay thế
/// crates-io bằng directory source và fail-closed khi thiếu crate; online chỉ
/// THÊM directory source, giữ crates-io làm nguồn chính vì directory source
/// không tải được. `--locked` giữ ở cả hai chế độ.)
pub(crate) fn build_rust_with_managed_source(
    root: &Path,
    optimizer_envs: std::collections::HashMap<String, String>,
    source_root: &Path,
    fetch: DependencyFetch,
) -> Result<()> {
    let start = Instant::now();
    info("Detected Rust library — building against the MagiCore-managed crate source...");
    if let Some(rustflags) = optimizer_envs.get("RUSTFLAGS") {
        mgc_ui::info(&format!("Applying RUSTFLAGS: {rustflags}"));
    }
    let source_path = source_root.to_str().ok_or_else(|| {
        anyhow::Error::new(super::error::RustBuildError::ManagedSourcePathNotUtf8)
    })?;
    let base = managed_source_config(fetch, source_path);
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

/// Build the `--config` prefix that wires the managed directory source.
///
/// Offline: `source.crates-io.replace-with` points crates-io at the managed
/// directory, so Cargo cannot reach the network and a missing crate fails.
/// Online: only `source.mgc-managed.directory` is declared — crates-io keeps
/// its real registry and stays able to download.
/// (Offline: `replace-with` trỏ crates-io vào thư mục managed nên Cargo không ra
/// mạng và crate thiếu sẽ fail. Online: chỉ khai báo directory của source managed
/// — crates-io giữ registry thật và vẫn tải được.)
pub(crate) fn managed_source_config(fetch: DependencyFetch, source_path: &str) -> Vec<String> {
    let source_config = format!(
        "source.{MANAGED_SOURCE_NAME}.directory={}",
        toml::Value::String(source_path.to_string())
    );
    let mut config = Vec::new();
    if fetch.is_offline() {
        config.push("--config".to_string());
        config.push(format!(
            "source.crates-io.replace-with=\"{MANAGED_SOURCE_NAME}\""
        ));
    }
    config.push("--config".to_string());
    config.push(source_config);
    config
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
