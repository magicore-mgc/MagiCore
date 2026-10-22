//! Runtime-specific optimizer file selection tests.
//! (Kiểm thử chọn file optimizer theo runtime.)

use crate::commands::optimizer::env_loader::load_optimizer_env;
use crate::commands::optimizer::runtime_detect::DetectedRuntime;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_load_optimizer_env_with_bun() {
    let temp = TempDir::new().unwrap();
    let optimizer_dir = temp.path().join(".mgc-optimizer");
    fs::create_dir(&optimizer_dir).unwrap();

    fs::write(
        optimizer_dir.join("bun_env.env"),
        "BUN_RUNTIME_TRANSPILER_CACHE_PATH=/tmp/bun-cache\n",
    )
    .unwrap();

    let vars = load_optimizer_env(temp.path(), &DetectedRuntime::Bun).unwrap();
    assert_eq!(
        vars.get("BUN_RUNTIME_TRANSPILER_CACHE_PATH"),
        Some(&"/tmp/bun-cache".to_string())
    );
}

#[test]
fn test_load_optimizer_env_runtime_mismatch() {
    let temp = TempDir::new().unwrap();
    let optimizer_dir = temp.path().join(".mgc-optimizer");
    fs::create_dir(&optimizer_dir).unwrap();

    // Write Bun config
    // (Ghi cấu hình Bun.)
    fs::write(
        optimizer_dir.join("bun_env.env"),
        "BUN_RUNTIME_TRANSPILER_CACHE_PATH=/tmp/bun-cache\n",
    )
    .unwrap();

    // Write Deno config
    // (Ghi cấu hình Deno.)
    fs::write(
        optimizer_dir.join("deno_env.env"),
        "DENO_V8_FLAGS=--max-old-space-size=4096\n",
    )
    .unwrap();

    // Request Deno runtime - should only load deno_env.env
    // (Yêu cầu Deno; chỉ nạp deno_env.env.)
    let vars = load_optimizer_env(temp.path(), &DetectedRuntime::Deno).unwrap();
    assert!(!vars.contains_key("BUN_RUNTIME_TRANSPILER_CACHE_PATH"));
    assert_eq!(
        vars.get("DENO_V8_FLAGS"),
        Some(&"--max-old-space-size=4096".to_string())
    );
}

#[test]
fn test_load_optimizer_env_ai_runtime() {
    let temp = TempDir::new().unwrap();
    let optimizer_dir = temp.path().join(".mgc-optimizer");
    fs::create_dir(&optimizer_dir).unwrap();

    // Write PyTorch config
    // (Ghi cấu hình PyTorch.)
    fs::write(
        optimizer_dir.join("pytorch_runtime.env"),
        "PYTORCH_CUDA_ALLOC_CONF=max_split_size_mb:512\n",
    )
    .unwrap();

    fs::write(
        optimizer_dir.join("pytorch_docker.env"),
        "PYTORCH_JIT_FALLBACK=1\n",
    )
    .unwrap();

    // Request PythonPyTorch runtime - should load both pytorch files
    // (Yêu cầu PythonPyTorch; nạp cả hai file PyTorch.)
    let vars = load_optimizer_env(temp.path(), &DetectedRuntime::PythonPyTorch).unwrap();
    assert_eq!(
        vars.get("PYTORCH_CUDA_ALLOC_CONF"),
        Some(&"max_split_size_mb:512".to_string())
    );
    assert_eq!(vars.get("PYTORCH_JIT_FALLBACK"), Some(&"1".to_string()));
}

#[test]
fn test_load_optimizer_env_lib_runtime() {
    let temp = TempDir::new().unwrap();
    let optimizer_dir = temp.path().join(".mgc-optimizer");
    fs::create_dir(&optimizer_dir).unwrap();

    // Write Rust lib config as TOML
    // (Ghi cấu hình Rust library theo định dạng TOML.)
    fs::write(
        optimizer_dir.join("rust_cargo_profile.toml"),
        r#"
[build]
jobs = 6
rustflags = ["-C", "target-cpu=native", "-C", "opt-level=3"]

[profile.dev]
opt-level = 2
codegen-units = 8
debug = 1

[profile.release]
opt-level = 3
lto = false
codegen-units = 1

[env]
RUST_BACKTRACE = "1"
"#,
    )
    .unwrap();

    // Request RustLib runtime
    // (Yêu cầu runtime RustLib.)
    let vars = load_optimizer_env(temp.path(), &DetectedRuntime::RustLib).unwrap();

    // Verify RUSTFLAGS extracted from TOML array
    // (Xác nhận RUSTFLAGS được lấy từ mảng TOML.)
    assert_eq!(
        vars.get("RUSTFLAGS"),
        Some(&"-C target-cpu=native -C opt-level=3".to_string())
    );

    assert_eq!(vars.get("CARGO_BUILD_JOBS"), Some(&"6".to_string()));
    assert_eq!(
        vars.get("CARGO_PROFILE_DEV_OPT_LEVEL"),
        Some(&"2".to_string())
    );
    assert_eq!(
        vars.get("CARGO_PROFILE_DEV_CODEGEN_UNITS"),
        Some(&"8".to_string())
    );
    assert_eq!(
        vars.get("CARGO_PROFILE_RELEASE_LTO"),
        Some(&"false".to_string())
    );

    // Verify [env] section parsed
    // (Xác nhận section [env] được parse.)
    assert_eq!(vars.get("RUST_BACKTRACE"), Some(&"1".to_string()));
}
