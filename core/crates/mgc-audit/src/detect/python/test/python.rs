//! Tests for the Python manifest detector.
//! Kiểm thử detector manifest Python.

#![allow(clippy::unwrap_used)]

use super::*;

fn tmp(tag: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("mgc-detect-{tag}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn python_step_fires_on_every_recognized_manifest() {
    for file in [
        "requirements.txt",
        "requirements-dev.txt",
        "pylock.toml",
        "pylock.dev.toml",
        "uv.lock",
        "pyproject.toml",
    ] {
        let dir = tmp(&format!("py-{}", file.replace('.', "-")));
        std::fs::write(dir.join(file), "").unwrap();
        let step = python_step(&dir).unwrap_or_else(|| panic!("{file} must yield a python step"));
        assert_eq!(step.ecosystem, "python");
        assert_eq!(step.scanner, "mgc-python-osv");
    }
}
