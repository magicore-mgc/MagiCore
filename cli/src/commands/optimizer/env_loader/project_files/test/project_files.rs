//! Optimizer project file safety test.
//! (Kiểm thử an toàn file project của optimizer.)

use crate::commands::optimizer::env_loader::load_optimizer_env;
use crate::commands::optimizer::runtime_detect::DetectedRuntime;
use std::fs;
use tempfile::TempDir;

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
