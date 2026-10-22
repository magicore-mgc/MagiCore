//! Tests for this runtime-detection owner.

use crate::commands::optimizer::runtime_detect::*;
use std::fs;
use tempfile::TempDir;

#[test]
fn test_detect_web_deno() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("deno.json"), "{}").unwrap();

    let runtimes = detect_runtimes(dir.path(), "web");
    assert_eq!(runtimes, vec![DetectedRuntime::Deno]);
}

#[test]
fn test_detect_web_bun() {
    let dir = TempDir::new().unwrap();
    fs::write(dir.path().join("bun.lockb"), "").unwrap();

    let runtimes = detect_runtimes(dir.path(), "web");
    assert_eq!(runtimes, vec![DetectedRuntime::Bun]);
}
