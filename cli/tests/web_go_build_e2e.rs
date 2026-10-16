//! Native Go backend build routing for Web-owned backend projects.
//! Khóa route build Go native cho backend thuộc project Web.

#![allow(clippy::unwrap_used)]

use std::process::{Command, Output};

fn write_go_backend(root: &std::path::Path) {
    std::fs::write(
        root.join("mgc.toml"),
        "name = \"go-backend-probe\"\nversion = \"0.1.0\"\necosystem = \"web\"\nmode = \"backend\"\nframeworks = [\"go\"]\n",
    )
    .unwrap();
    std::fs::write(root.join(".mgc.core"), "web\n").unwrap();
    std::fs::write(
        root.join("go.mod"),
        "module example.test/go-backend-probe\n\ngo 1.23\n",
    )
    .unwrap();
    std::fs::write(root.join("main.go"), "package main\n\nfunc main() {}\n").unwrap();
}

fn run_build(root: &std::path::Path, flags: &[&str]) -> Output {
    let home = root.join("home");
    std::fs::create_dir_all(&home).unwrap();
    let mut command = Command::new(env!("CARGO_BIN_EXE_mgc"));
    command
        .arg("build")
        .args(flags)
        .current_dir(root)
        .env("HOME", &home)
        .env("USERPROFILE", &home)
        .env("MGC_DATA_DIR", root.join("data"))
        .env("MGC_CACHE_DIR", root.join("cache"))
        .env("MAGICORE_STORE_ROOT", root.join("store"))
        .env("GOPROXY", "https://example.invalid")
        .env("GOTOOLCHAIN", "auto");
    for name in [
        "MAGICORE_WEB_REGISTRY_URL",
        "MAGICORE_WEB_REGISTRY_TOKEN",
        "MAGICORE_WEB_ALLOWED_REGISTRIES",
    ] {
        command.env_remove(name);
    }
    command.output().expect("run Go backend build command")
}

#[test]
fn go_backend_rejects_javascript_only_flags_before_bundler_dispatch() {
    for flags in [["--target", "native"], ["--compat-runner", "npm"]] {
        let root = tempfile::tempdir().unwrap();
        write_go_backend(root.path());

        let output = run_build(root.path(), &flags);
        let message = format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );

        assert!(!output.status.success(), "unsupported flag must fail");
        assert!(
            message.contains(flags[0]) && message.contains("go backend"),
            "expected precise Go flag error, got: {message}"
        );
        assert!(
            !message.contains("Could not find entry point"),
            "a Go project must not reach the JavaScript bundler: {message}"
        );
    }
}

#[test]
#[ignore = "Requires Go SDK on PATH; run explicitly in Go backend build qualification"]
fn go_backend_build_uses_mgc_module_store_and_central_build_cache() {
    let sdk = Command::new("go")
        .arg("version")
        .output()
        .expect("Go SDK must be installed for this qualification");
    assert!(sdk.status.success(), "Go SDK must be usable");

    let root = tempfile::tempdir().unwrap();
    write_go_backend(root.path());
    let output = run_build(root.path(), &[]);
    let message = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        output.status.success(),
        "Go backend build failed: {message}"
    );
    assert!(message.contains("Building Go Web backend"));
    assert!(root.path().join("store/go").is_dir());
    assert!(root.path().join("data/cache/go/build").is_dir());
    assert!(!root.path().join(".magicore/cache/go/pkg/mod").exists());

    let binary = root.path().join(if cfg!(windows) {
        ".magicore/build/go/server.exe"
    } else {
        ".magicore/build/go/server"
    });
    assert!(binary.is_file(), "successful build must produce a binary");
    assert!(Command::new(binary).status().unwrap().success());
}
