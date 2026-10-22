//! Cargo TOML fail-closed validation test.
//! (Kiểm thử từ chối cấu hình Cargo TOML không hợp lệ.)

use crate::commands::optimizer::env_loader::load_optimizer_env;
use crate::commands::optimizer::runtime_detect::DetectedRuntime;
use std::fs;
use tempfile::TempDir;

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
