//! Local project discovery tests.
//! Test dò project local.

use super::super::{copy_local_project, discover_local_projects};
use std::fs;
use std::path::PathBuf;

fn sandbox(tag: &str) -> PathBuf {
    let root = std::env::temp_dir().join(format!(
        "mgc-local-{tag}-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    ));
    fs::create_dir_all(&root).expect("sandbox");
    root
}

fn write_project(root: &std::path::Path, ecosystem: &str, framework: &str, marker: &str) {
    fs::create_dir_all(root).expect("project");
    fs::write(
        root.join("mgc.toml"),
        format!("name = \"p\"\nversion = \"0.1.0\"\necosystem = \"{ecosystem}\"\nframeworks = [\"{framework}\"]\n"),
    )
    .expect("manifest");
    fs::write(root.join(marker), "x").expect("marker");
}

#[test]
fn discovers_by_manifest_and_marker_not_folder_name() {
    let root = sandbox("discover");
    // Folder name says django but the manifest/marker say otherwise.
    write_project(
        &root.join("looks-like-django"),
        "web",
        "fastapi",
        "pyproject.toml",
    );
    write_project(&root.join("anything"), "web", "django", "manage.py");
    write_project(&root.join("other-core"), "lib", "rust", "Cargo.toml");

    let found = discover_local_projects(&root, "web", "django").expect("discover");
    assert_eq!(found.len(), 1);
    assert!(found[0].root.ends_with("anything"));
    assert_eq!(found[0].frameworks, vec!["django".to_string()]);
    fs::remove_dir_all(&root).ok();
}

#[test]
fn ignores_projects_without_framework_marker() {
    let root = sandbox("marker");
    let project = root.join("claims-django");
    fs::create_dir_all(&project).expect("project");
    fs::write(
        project.join("mgc.toml"),
        "name = \"p\"\nversion = \"0.1.0\"\necosystem = \"web\"\nframeworks = [\"django\"]\n",
    )
    .expect("manifest");

    let found = discover_local_projects(&root, "web", "django").expect("discover");
    assert!(found.is_empty(), "manifest alone is not proof");
    fs::remove_dir_all(&root).ok();
}

#[test]
fn copy_excludes_generated_trees_git_and_env() {
    let root = sandbox("copy");
    let source = root.join("source");
    write_project(&source, "web", "django", "manage.py");
    for generated in ["node_modules", ".venv", "target", "dist", "build", ".git"] {
        fs::create_dir_all(source.join(generated)).expect("generated");
        fs::write(source.join(generated).join("leftover"), "x").expect("file");
    }
    fs::write(source.join(".env"), "SECRET=1").expect("env");
    let target = root.join("target");

    copy_local_project(&source, &target).expect("copy");

    for skipped in [
        "node_modules",
        ".venv",
        "target",
        "dist",
        "build",
        ".git",
        ".env",
    ] {
        assert!(
            !target.join(skipped).exists(),
            "{skipped} must not be copied"
        );
    }
    assert!(target.join("mgc.toml").exists());
    assert!(target.join("manage.py").exists());
    fs::remove_dir_all(&root).ok();
}

#[test]
fn missing_root_is_an_empty_result_not_an_error() {
    let root = sandbox("missing");
    let found = discover_local_projects(&root.join("absent"), "web", "django").expect("discover");
    assert!(found.is_empty());
    fs::remove_dir_all(&root).ok();
}
