//! Backend launch resolution tests — kiểm thử resolve launch backend.
//! The launch must carry the runner the server half needs and the endpoint
//! environment the single-framework lane already established.
//! Launch phải mang runner mà nửa server cần và môi trường endpoint mà lane một
//! framework đã đặt.

use std::path::Path;

use super::{build_backend_launch, no_backend_start};
use crate::commands::compat::CompatMode;
use crate::commands::core::dev_port::PORT_WEB_BE;

#[test]
fn python_half_launches_with_the_shared_endpoint_env() {
    let dir = tempfile::tempdir().expect("temp dir");
    write(dir.path(), "main.py", "print('x')\n");
    write(
        dir.path(),
        "pyproject.toml",
        "[project]\nname = \"api-server\"\n",
    );

    let launch = build_backend_launch(dir.path(), &CompatMode::Native, None)
        .expect("python half resolves a launch");

    assert!(
        launch.program.to_string_lossy().starts_with("python"),
        "python interpreter expected: {}",
        launch.program.display()
    );
    assert_eq!(
        launch.args,
        vec![std::ffi::OsString::from("main.py")],
        "the entrypoint file is the only argument"
    );
    assert_env(&launch.envs, "PORT", &PORT_WEB_BE.to_string());
    assert_env(&launch.envs, "SERVER_PORT", &PORT_WEB_BE.to_string());
    assert!(
        launch.envs.iter().any(|(key, _)| key == "HOST"),
        "HOST is exported so browser preflight is answered"
    );
}

#[test]
fn rust_and_go_halves_launch_their_runners() {
    let rust = tempfile::tempdir().expect("temp dir");
    write(rust.path(), "Cargo.toml", "[package]\nname = \"server\"\n");
    write(rust.path(), "src/main.rs", "fn main() {}\n");
    let launch = build_backend_launch(rust.path(), &CompatMode::Native, None)
        .expect("rust half resolves a launch");
    assert_eq!(launch.program.to_string_lossy(), "cargo");
    assert_eq!(
        launch.args.first().map(|a| a.to_string_lossy()),
        Some("run".into())
    );

    let go = tempfile::tempdir().expect("temp dir");
    write(go.path(), "go.mod", "module server\n");
    write(go.path(), "main.go", "package main\n");
    let launch = build_backend_launch(go.path(), &CompatMode::Native, None)
        .expect("go half resolves a launch");
    assert_eq!(launch.program.to_string_lossy(), "go");
    assert_eq!(
        launch.args.first().map(|a| a.to_string_lossy()),
        Some("run".into())
    );
}

#[test]
fn empty_server_directory_reports_what_is_missing() {
    let dir = tempfile::tempdir().expect("temp dir");
    let error = build_backend_launch(dir.path(), &CompatMode::Native, None)
        .expect_err("an empty server directory cannot launch");
    let message = error.to_string();
    assert!(message.contains("no backend manifest"), "{message}");
    assert!(message.contains("pyproject.toml"), "{message}");
}

#[test]
fn startup_error_names_the_directory_and_the_manifests() {
    let dir = tempfile::tempdir().expect("temp dir");
    let message = no_backend_start(dir.path()).to_string();
    assert!(
        message.contains(&dir.path().display().to_string()),
        "{message}"
    );
    assert!(message.contains("Cargo.toml"), "{message}");
}

fn assert_env(envs: &[(std::ffi::OsString, std::ffi::OsString)], key: &str, value: &str) {
    assert!(
        envs.iter()
            .any(|(name, candidate)| name == key && candidate == value),
        "{key}={value} missing from {:?}",
        envs
    );
}

fn write(dir: &Path, name: &str, contents: &str) {
    let path = dir.join(name);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("parent directory");
    }
    std::fs::write(path, contents).expect("write fixture file");
}
