//! Optimizer loader directory behavior test.
//! (Kiểm thử hành vi thư mục của optimizer loader.)

use crate::commands::optimizer::env_loader::load_optimizer_env;
use crate::commands::optimizer::runtime_detect::DetectedRuntime;
use tempfile::TempDir;

#[test]
fn test_load_optimizer_env_no_dir() {
    let temp = TempDir::new().unwrap();
    let vars = load_optimizer_env(temp.path(), &DetectedRuntime::Bun).unwrap();
    assert!(vars.is_empty());
}
