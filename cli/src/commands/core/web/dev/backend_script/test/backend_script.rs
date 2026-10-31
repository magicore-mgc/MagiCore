//! Backend-half start-command tests — kiểm thử lệnh khởi động nửa backend.
//! Each generated server half must resolve to the runner that starts it; a
//! directory with no backend marker must resolve to nothing rather than a
//! guessed command.
//! Mỗi nửa server sinh ra phải resolve đúng runner khởi động nó; thư mục không
//! có marker backend phải resolve về không thay vì đoán lệnh.

use std::path::Path;

use super::super::{BackendScript, is_backend_half, resolve_backend_script};

#[test]
fn directory_without_a_backend_marker_is_not_a_half() {
    let dir = tempfile::tempdir().expect("temp dir");
    assert!(!is_backend_half(dir.path()));
    assert_eq!(resolve_backend_script(dir.path()), None);
}

#[test]
fn fastapi_half_starts_the_python_entrypoint() {
    let dir = tempfile::tempdir().expect("temp dir");
    write(dir.path(), "main.py", "app = None\n");
    write(
        dir.path(),
        "pyproject.toml",
        "[project]\nname = \"api-server\"\n",
    );

    assert!(is_backend_half(dir.path()));
    assert_eq!(
        resolve_backend_script(dir.path()),
        Some(BackendScript::Runner {
            program: python_launcher(),
            args: &["main.py"],
            label: "python server",
        })
    );
}

#[test]
fn flask_and_django_halves_start_their_entrypoints() {
    let flask = tempfile::tempdir().expect("temp dir");
    write(flask.path(), "app.py", "app = None\n");
    write(
        flask.path(),
        "pyproject.toml",
        "[project]\nname = \"api-server\"\n",
    );
    assert_eq!(
        resolve_backend_script(flask.path()),
        Some(BackendScript::Runner {
            program: python_launcher(),
            args: &["app.py"],
            label: "flask server",
        })
    );

    let django = tempfile::tempdir().expect("temp dir");
    write(django.path(), "manage.py", "def main(): pass\n");
    write(
        django.path(),
        "pyproject.toml",
        "[project]\nname = \"api-server\"\n",
    );
    assert_eq!(
        resolve_backend_script(django.path()),
        Some(BackendScript::Runner {
            program: python_launcher(),
            args: &["manage.py", "runserver"],
            label: "django server",
        })
    );
}

#[test]
fn rust_and_go_halves_start_their_own_runners() {
    let rust = tempfile::tempdir().expect("temp dir");
    write(rust.path(), "Cargo.toml", "[package]\nname = \"server\"\n");
    write(rust.path(), "src/main.rs", "fn main() {}\n");
    assert_eq!(
        resolve_backend_script(rust.path()),
        Some(BackendScript::Runner {
            program: "cargo",
            args: &["run", "--quiet", "--locked"],
            label: "axum server",
        })
    );

    let go = tempfile::tempdir().expect("temp dir");
    write(go.path(), "go.mod", "module server\n");
    write(go.path(), "main.go", "package main\n");
    assert_eq!(
        resolve_backend_script(go.path()),
        Some(BackendScript::Runner {
            program: "go",
            args: &["run", "."],
            label: "gin server",
        })
    );
}

#[test]
fn node_half_keeps_a_declared_script() {
    let dir = tempfile::tempdir().expect("temp dir");
    write(
        dir.path(),
        "package.json",
        r#"{"name": "server", "scripts": {"dev": "node src/server.js"}}"#,
    );
    assert_eq!(
        resolve_backend_script(dir.path()),
        Some(BackendScript::NodeScript("dev"))
    );
}

#[test]
fn node_half_without_a_start_script_is_refused() {
    let dir = tempfile::tempdir().expect("temp dir");
    write(dir.path(), "package.json", r#"{"name": "server"}"#);
    assert!(
        is_backend_half(dir.path()),
        "the manifest still marks a backend half"
    );
    assert_eq!(
        resolve_backend_script(dir.path()),
        None,
        "a manifest with no start script must not resolve to a guessed command"
    );
}

#[cfg(windows)]
fn python_launcher() -> &'static str {
    "python"
}

#[cfg(not(windows))]
fn python_launcher() -> &'static str {
    "python3"
}

fn write(dir: &Path, name: &str, contents: &str) {
    let path = dir.join(name);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("parent directory");
    }
    std::fs::write(path, contents).expect("write fixture file");
}
