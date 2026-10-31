//! Split server-half toolchain tests — kiểm thử toolchain của nửa server tách rời.
//! A project's `server/` half runs on its own runner; readiness must cover it.
//! Nửa `server/` của project chạy bằng runner riêng; readiness phải bao phủ nó.

use std::path::Path;

use super::super::requirements_for_project;
use super::super::split_server_half_tools;

#[test]
fn each_generated_half_maps_to_its_runner() {
    for (manifest, entry, expected) in [
        ("pyproject.toml", "main.py", "python3"),
        ("pyproject.toml", "app.py", "python3"),
        ("Cargo.toml", "src/main.rs", "cargo"),
        ("go.mod", "main.go", "go"),
    ] {
        let root = tempfile::tempdir().expect("temp dir");
        write(root.path(), &format!("server/{manifest}"), "fixture");
        write(root.path(), &format!("server/{entry}"), "fixture");
        assert_eq!(
            split_server_half_tools(root.path()),
            &[expected],
            "{manifest} + {entry} runs on {expected}"
        );
    }
}

#[test]
fn project_without_a_server_half_needs_only_the_core_row() {
    let root = tempfile::tempdir().expect("temp dir");
    write(root.path(), "package.json", "{}");
    assert!(split_server_half_tools(root.path()).is_empty());

    // A manifest with nothing to run is not a half.
    // (Manifest không có gì để chạy không phải nửa server.)
    let bare = tempfile::tempdir().expect("temp dir");
    write(bare.path(), "server/pyproject.toml", "fixture");
    assert!(split_server_half_tools(bare.path()).is_empty());
}

#[test]
fn web_requirements_add_the_split_runner_without_duplicating_node() {
    let root = tempfile::tempdir().expect("temp dir");
    write(root.path(), "server/pyproject.toml", "fixture");
    write(root.path(), "server/main.py", "fixture");

    let requirements = requirements_for_project("web", root.path());
    let binaries: Vec<&str> = requirements
        .iter()
        .map(|requirement| requirement.binary)
        .collect();
    assert!(binaries.contains(&"node"), "{binaries:?}");
    assert!(binaries.contains(&"python3"), "{binaries:?}");
    // The split runner is tagged as required by the same core.
    // (Runner tách rời được gắn cùng core yêu cầu.)
    assert!(
        requirements
            .iter()
            .filter(|requirement| requirement.binary == "python3")
            .all(|requirement| requirement.core == "web")
    );
}

#[test]
fn non_web_cores_keep_their_table_row() {
    let root = tempfile::tempdir().expect("temp dir");
    write(root.path(), "server/go.mod", "fixture");
    write(root.path(), "server/main.go", "fixture");

    let program = requirements_for_project("program-something", root.path());
    assert!(program.is_empty(), "an unknown core has no row");

    let ai = requirements_for_project("ai", root.path());
    assert_eq!(ai.len(), 1, "the ai row is untouched by a stray server dir");
    assert_eq!(ai[0].binary, "python3");
}

fn write(dir: &Path, relative: &str, contents: &str) {
    let path = dir.join(relative);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("parent directory");
    }
    std::fs::write(path, contents).expect("write fixture file");
}
