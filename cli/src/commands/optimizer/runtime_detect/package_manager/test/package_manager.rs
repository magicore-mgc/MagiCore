//! Tests for this runtime-detection owner.

use crate::commands::optimizer::runtime_detect::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_detect_web_nodejs_pnpm() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("package.json"), "{}").unwrap();
    fs::write(dir.path().join("pnpm-lock.yaml"), "").unwrap();

    let runtimes = detect_runtimes(dir.path(), "web");
    assert_eq!(
        runtimes,
        vec![DetectedRuntime::NodeJs {
            package_manager: PackageManager::Pnpm
        }]
    );
}
