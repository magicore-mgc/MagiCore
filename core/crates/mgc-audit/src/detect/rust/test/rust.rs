//! Tests for the Rust manifest detector.
//! Kiểm thử detector manifest Rust.

#![allow(clippy::unwrap_used)]

use super::*;

fn tmp(tag: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("mgc-detect-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn rust_step_fires_on_cargo_toml() {
    let dir = tmp("rust");
    std::fs::write(dir.join("Cargo.toml"), "[package]\nname = \"x\"\n").unwrap();
    let step = rust_step(&dir).expect("Cargo.toml must yield a rust step");
    assert_eq!(step.ecosystem, "rust");
    assert_eq!(step.scanner, "mgc-rust-osv");
}
