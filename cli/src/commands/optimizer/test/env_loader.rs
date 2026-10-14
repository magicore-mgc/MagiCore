// Tests for env_loader — extracted from inline tests per RULE §5
// Tests cho env_loader — tách từ inline tests theo RULE §5

use super::super::env_loader::*;
use crate::commands::optimizer::runtime_detect::DetectedRuntime;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_parse_env_file() {
    let temp = TempDir::new().unwrap();
    let env_file = temp.path().join("test.env");

    fs::write(
        &env_file,
        r#"
# Comment line
KEY1=value1
KEY2="quoted value"
KEY3='single quoted'
EMPTY_KEY=

# Another comment
KEY4=value with spaces
"#,
    )
    .unwrap();

    let vars = parse_env_file(&env_file).unwrap();

    assert_eq!(vars.get("KEY1"), Some(&"value1".to_string()));
    assert_eq!(vars.get("KEY2"), Some(&"quoted value".to_string()));
    assert_eq!(vars.get("KEY3"), Some(&"single quoted".to_string()));
    assert_eq!(vars.get("EMPTY_KEY"), Some(&"".to_string()));
    assert_eq!(vars.get("KEY4"), Some(&"value with spaces".to_string()));
}

#[test]
fn malformed_quote_returns_line_error_without_exposing_value() {
    let temp = TempDir::new().unwrap();
    let env_file = temp.path().join("malformed.env");
    fs::write(&env_file, "SAFE=value\nAUDIT_VALUE=\"\n").unwrap();

    let error = parse_env_file(&env_file).unwrap_err().to_string();
    assert!(error.contains("line 2"));
    assert!(!error.contains("AUDIT_VALUE"));
    assert!(!error.contains('"'));
}

#[test]
fn trailing_quote_without_an_opening_quote_is_preserved() {
    let temp = TempDir::new().unwrap();
    let env_file = temp.path().join("literal-quote.env");
    fs::write(&env_file, "LITERAL=value\"\n").unwrap();

    let vars = parse_env_file(&env_file).unwrap();
    assert_eq!(vars.get("LITERAL"), Some(&"value\"".to_string()));
}

#[test]
fn test_load_optimizer_env_no_dir() {
    let temp = TempDir::new().unwrap();
    let vars = load_optimizer_env(temp.path(), &DetectedRuntime::Bun).unwrap();
    assert!(vars.is_empty());
}

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
    fs::write(
        optimizer_dir.join("bun_env.env"),
        "BUN_RUNTIME_TRANSPILER_CACHE_PATH=/tmp/bun-cache\n",
    )
    .unwrap();

    // Write Deno config
    fs::write(
        optimizer_dir.join("deno_env.env"),
        "DENO_V8_FLAGS=--max-old-space-size=4096\n",
    )
    .unwrap();

    // Request Deno runtime - should only load deno_env.env
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
    let vars = load_optimizer_env(temp.path(), &DetectedRuntime::RustLib).unwrap();

    // Verify RUSTFLAGS extracted from TOML array
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
    assert_eq!(vars.get("RUST_BACKTRACE"), Some(&"1".to_string()));
}

#[test]
fn candle_runtime_loads_both_runtime_and_cargo_optimizer_settings() {
    let temp = TempDir::new().unwrap();
    let optimizer_dir = temp.path().join(".mgc-optimizer");
    fs::create_dir(&optimizer_dir).unwrap();
    fs::write(
        optimizer_dir.join("candle_runtime.env"),
        "RAYON_NUM_THREADS=4\nRUST_BACKTRACE=1\n",
    )
    .unwrap();
    fs::write(
        optimizer_dir.join("candle_cargo.toml"),
        "[build]\njobs = 3\n[profile.dev]\nopt-level = 1\n[profile.release]\nopt-level = 3\nlto = \"thin\"\n",
    )
    .unwrap();

    let vars = load_optimizer_env(temp.path(), &DetectedRuntime::RustCandle).unwrap();
    assert_eq!(vars.get("RAYON_NUM_THREADS"), Some(&"4".to_string()));
    assert_eq!(vars.get("CARGO_BUILD_JOBS"), Some(&"3".to_string()));
    assert_eq!(
        vars.get("CARGO_PROFILE_DEV_OPT_LEVEL"),
        Some(&"1".to_string())
    );
    assert_eq!(
        vars.get("CARGO_PROFILE_RELEASE_LTO"),
        Some(&"\"thin\"".to_string())
    );
}

#[test]
fn invalid_or_process_control_optimizer_settings_fail_closed() {
    let temp = TempDir::new().unwrap();
    let optimizer_dir = temp.path().join(".mgc-optimizer");
    fs::create_dir(&optimizer_dir).unwrap();
    let path = optimizer_dir.join("rust_cargo_profile.toml");

    fs::write(&path, "[profile.release]\nlto = \"false\"\n").unwrap();
    assert!(load_optimizer_env(temp.path(), &DetectedRuntime::RustLib).is_err());

    fs::write(&path, "[build]\njobs = 0\n").unwrap();
    assert!(load_optimizer_env(temp.path(), &DetectedRuntime::RustLib).is_err());

    fs::write(&path, "[env]\nPATH = \"/tmp/injected\"\n").unwrap();
    assert!(load_optimizer_env(temp.path(), &DetectedRuntime::RustLib).is_err());
}

#[cfg(unix)]
#[test]
fn optimizer_config_symlink_is_rejected() {
    use std::os::unix::fs::symlink;

    let temp = TempDir::new().unwrap();
    let outside = temp.path().join("outside");
    let optimizer_dir = temp.path().join(".mgc-optimizer");
    fs::create_dir(&outside).unwrap();
    fs::write(
        outside.join("rust_cargo_profile.toml"),
        "[build]\njobs = 2\n",
    )
    .unwrap();
    symlink(&outside, &optimizer_dir).unwrap();

    assert!(load_optimizer_env(temp.path(), &DetectedRuntime::RustLib).is_err());
}
