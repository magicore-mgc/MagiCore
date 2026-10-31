use anyhow::Result;
use std::path::Path;
use std::time::Instant;

use mgc_ui::info;

use crate::commands::build::implementation::DependencyFetch;
use crate::commands::build::implementation::shared::toolchain_remedy::{
    ToolchainContext, with_remedy,
};

/// Cargo's own target override; it selects the compile target without any
/// `--target` argument on the command line.
/// Biến ghi đè target của Cargo; nó chọn target biên dịch mà không cần
/// `--target` trên dòng lệnh.
const CARGO_BUILD_TARGET_ENV_VAR: &str = "CARGO_BUILD_TARGET";

/// Cargo's build configuration path; a `build.target` key there also selects
/// the compile target.
/// Đường dẫn cấu hình build của Cargo; khóa `build.target` ở đó cũng chọn
/// target biên dịch.
const CARGO_BUILD_CONFIG_ENV_VAR: &str = "CARGO_BUILD_CONFIG";

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
    let rust_target = rust_target_context();
    match mgc_exec::prelude::run(
        "cargo",
        &cargo_build_args(fetch, &[], has_lock(root)),
        &opts,
    ) {
        Ok(_) => {}
        Err(error) => {
            return Err(build_rust_with_managed_source_failure(
                error,
                rust_target.as_deref(),
            ));
        }
    }

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
    let args = cargo_build_args(fetch, &base, has_lock(root));
    let opts = mgc_exec::prelude::ExecOptions {
        cwd: Some(root.to_path_buf()),
        log_path: Some(root.join(".magicore").join("exec.log")),
        env: optimizer_envs.into_iter().collect(),
        clean_env: false,
        execution_scope: Some(mgc_exec::allowlist::ExecutionScope::BuildRunner),
        ..Default::default()
    };
    let rust_target = rust_target_context();
    match mgc_exec::prelude::run("cargo", &args, &opts) {
        Ok(_) => {}
        Err(error) => {
            return Err(build_rust_with_managed_source_failure(
                error,
                rust_target.as_deref(),
            ));
        }
    }
    mgc_ui::success(&format!(
        "Rust library build completed in {:?}",
        start.elapsed()
    ));
    Ok(())
}

/// Rewrite a failed `cargo build` so a missing Rust target names its fix.
///
/// A target that is not installed fails deep inside the compiler with
/// `can't find crate for core`, which names neither the missing target nor the
/// fix. Every Rust build lane funnels its failure through here, so `web`, `lib`,
/// `iot` and `game` report the same `rustup target add` command; a failure the
/// classifier does not recognise is returned unchanged.
/// (Target chưa cài hỏng sâu trong compiler với `can't find crate for core`,
/// không nói thiếu target nào hay sửa sao. Mọi lane build Rust đều đưa lỗi qua
/// đây nên `web`, `lib`, `iot`, `game` báo cùng lệnh `rustup target add`; lỗi
/// không phân loại được trả nguyên vẹn.)
fn build_rust_with_managed_source_failure(
    error: anyhow::Error,
    rust_target: Option<&str>,
) -> anyhow::Error {
    let detail = error.to_string();
    let context = ToolchainContext {
        rust_target,
        python: None,
    };
    anyhow::anyhow!(with_remedy(detail.clone(), &detail, context))
}

/// The Rust triple a failing `cargo build` was compiling for, if knowable.
///
/// Resolution happens before the spawn so the spawn itself stays a single
/// inventoried `cargo` call. Cargo's order is `--target`, then
/// `CARGO_BUILD_TARGET`, then `build.target` from the effective Cargo config,
/// then the host — reading only the command line named the host triple while
/// cargo reported the real cross target as missing.
/// (Phân giải trước khi spawn để lệnh spawn vẫn là một lệnh `cargo` duy nhất đã
/// vào danh mục. Thứ tự Cargo: `--target`, `CARGO_BUILD_TARGET`, `build.target`
/// trong Cargo config hiệu lực, rồi host — chỉ đọc dòng lệnh sẽ nêu host triple
/// trong khi cargo báo chính target chéo là thứ còn thiếu.)
fn rust_target_context() -> Option<String> {
    // Only a target the user or project explicitly selected is reported. Without
    // one, cargo compiles for the host, and a missing host std is not fixable by
    // `rustup target add <host>` — the remedy then names the command shape
    // instead of inventing a triple.
    // (Chỉ báo target mà user/project đã chọn rõ ràng. Không có thì cargo biên
    // dịch cho host, và thiếu std của host không sửa được bằng
    // `rustup target add <host>` — câu khắc phục khi đó nêu dạng lệnh, không đoán
    // triple.)
    requested_rust_target().or_else(configured_rust_target)
}

/// The triple the process was started with `--target` / `--target=<triple>`.
/// Triple mà process được khởi chạy với `--target` / `--target=<triple>`.
fn requested_rust_target() -> Option<String> {
    let mut arguments = std::env::args().skip(1);
    loop {
        let argument = arguments.next()?;
        if let Some(target) = argument.strip_prefix("--target=") {
            return Some(target.to_string()).filter(|target| !target.is_empty());
        }
        if argument == "--target" {
            return arguments.next().filter(|target| !target.is_empty());
        }
    }
}

/// The triple Cargo will actually compile for, when it was configured through
/// the environment or a Cargo config file instead of `--target`.
///
/// Cargo resolves `--target` first, then `CARGO_BUILD_TARGET`, then
/// `build.target` from the effective Cargo config, then the host. Reading only
/// the command line named the host triple in the remedy while cargo reported
/// the real cross target as missing.
/// (Cargo ưu tiên `--target`, rồi `CARGO_BUILD_TARGET`, rồi `build.target`
/// trong Cargo config hiệu lực, rồi host. Chỉ đọc dòng lệnh khiến câu khắc phục
/// nêu host triple trong khi cargo báo chính target chéo là thứ còn thiếu.)
fn configured_rust_target() -> Option<String> {
    if let Some(target) = std::env::var_os(CARGO_BUILD_TARGET_ENV_VAR)
        .and_then(|value| value.into_string().ok())
        .filter(|value| !value.trim().is_empty())
    {
        return Some(target.trim().to_string());
    }
    let config_path = std::env::var_os(CARGO_BUILD_CONFIG_ENV_VAR)
        .and_then(|value| value.into_string().ok())
        .filter(|value| !value.trim().is_empty())
        .map(std::path::PathBuf::from)
        .or_else(|| Some(std::path::PathBuf::from(".cargo").join("config.toml")));
    let config_path = config_path?;
    let content = std::fs::read_to_string(&config_path).ok()?;
    let value: toml::Value = toml::from_str(&content).ok()?;
    value
        .get("build")
        .and_then(|build| build.get("target"))
        // `build.target` may be a list for multi-target builds; the first entry
        // is the one Cargo reports first, so it is the one the user must add.
        // (`build.target` có thể là danh sách; phần tử đầu là target cargo báo
        // trước, nên cũng là target user phải cài.)
        .and_then(|target| match target {
            toml::Value::String(target) => Some(target.clone()),
            toml::Value::Array(targets) => targets
                .first()
                .and_then(toml::Value::as_str)
                .map(str::to_string),
            _ => None,
        })
        .filter(|target| !target.trim().is_empty())
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
pub(crate) fn cargo_build_args(
    fetch: DependencyFetch,
    prefix: &[String],
    has_lock: bool,
) -> Vec<String> {
    let mut args = prefix.to_vec();
    args.push("build".to_string());
    // `--locked` only when a Cargo.lock exists: a freshly scaffolded Rust
    // project has none, and passing --locked would make cargo refuse to
    // create it, so the build could never run.
    // Chỉ dùng `--locked` khi đã có Cargo.lock: project Rust vừa scaffold chưa
    // có, thêm --locked sẽ khiến cargo từ chối tạo lock và build không chạy được.
    if has_lock {
        args.push("--locked".to_string());
    }
    if fetch.is_offline() {
        args.push("--offline".to_string());
    }
    args
}

/// Whether the project root already has a Cargo.lock.
/// Project root đã có Cargo.lock chưa.
fn has_lock(root: &std::path::Path) -> bool {
    root.join("Cargo.lock").is_file()
}

#[cfg(test)]
#[path = "test/mod.rs"]
mod tests;
