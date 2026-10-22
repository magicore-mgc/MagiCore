//! Cargo profile environment integration test.
//! (Kiểm thử tích hợp profile Cargo với biến môi trường.)

use crate::commands::optimizer::env_loader::load_optimizer_env;
use crate::commands::optimizer::runtime_detect::DetectedRuntime;
use std::fs;
use tempfile::TempDir;

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
