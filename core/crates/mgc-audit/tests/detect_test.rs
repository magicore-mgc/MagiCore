//! Public API smoke test for the shared detector facade.
//! Smoke test API công khai cho facade detector dùng chung.

#![allow(clippy::unwrap_used)]

use mgc_audit::detect::{dotnet_step, go_step, java_step, python_step, rust_step};

fn tmp(tag: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("mgc-detect-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn empty_project_yields_no_steps() {
    let dir = tmp("empty");
    assert!(rust_step(&dir).is_none());
    assert!(python_step(&dir).is_none());
    assert!(go_step(&dir).is_none());
    assert!(java_step(&dir).is_none());
    assert!(dotnet_step(&dir).is_none());
}
