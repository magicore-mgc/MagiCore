//! Tests for the Go manifest detector.
//! Kiểm thử detector manifest Go.

#![allow(clippy::unwrap_used)]

use super::*;

fn tmp(tag: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("mgc-detect-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn go_step_fires_on_go_mod() {
    let dir = tmp("go");
    std::fs::write(dir.join("go.mod"), "module example.com/x\n").unwrap();
    let step = go_step(&dir).expect("go.mod must yield a go step");
    assert_eq!(step.ecosystem, "go");
    assert_eq!(step.scanner, "mgc-go-osv");
}
