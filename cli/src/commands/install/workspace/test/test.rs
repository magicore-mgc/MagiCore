#![cfg(test)]
#![allow(clippy::unwrap_used)]

use super::super::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_discover_workspace_projects_for_monorepo_root() {
    let dir = tempdir().unwrap();
    fs::write(
        dir.path().join("magicore.workspace.toml"),
        r#"
version = 1
mode = "monorepo"

[layout]
apps_dir = "apps"
packages_dir = "packages"
"#,
    )
    .unwrap();

    let frontend = dir.path().join("apps").join("frontend");
    let backend = dir.path().join("apps").join("backend");
    let contracts = dir.path().join("packages").join("contracts");
    fs::create_dir_all(&frontend).unwrap();
    fs::create_dir_all(&backend).unwrap();
    fs::create_dir_all(&contracts).unwrap();
    fs::write(frontend.join("package.json"), "{}").unwrap();
    fs::write(contracts.join("package.json"), "{}").unwrap();

    let workspaces = discover_workspace_projects(dir.path())
        .unwrap()
        .expect("should detect monorepo");

    assert_eq!(workspaces, vec![frontend, contracts]);
    assert!(!workspaces.contains(&backend));
}

#[test]
fn test_discover_workspace_projects_mix_cores() {
    let dir = tempdir().unwrap();
    fs::write(
        dir.path().join("magicore.workspace.toml"),
        r#"
mode = "monorepo"
[layout]
apps_dir = "apps"
packages_dir = "packages"
"#,
    )
    .unwrap();

    let web = dir.path().join("apps/web");
    fs::create_dir_all(&web).unwrap();
    fs::write(web.join("package.json"), "{}").unwrap();

    let lib = dir.path().join("packages/rustlib");
    fs::create_dir_all(lib.join("src")).unwrap();
    fs::write(lib.join("Cargo.toml"), "[package]\nname = \"rustlib\"\n").unwrap();

    let ignored = dir.path().join("packages/not-a-project");
    fs::create_dir_all(&ignored).unwrap();
    fs::write(ignored.join("notes.txt"), "x").unwrap();

    let mut workspaces = discover_workspace_projects(dir.path()).unwrap().unwrap();
    workspaces.sort();
    let normalized: Vec<String> = workspaces
        .iter()
        .map(|p| {
            p.strip_prefix(dir.path())
                .unwrap_or(p)
                .to_string_lossy()
                .to_string()
        })
        .collect();
    assert_eq!(normalized, vec!["apps/web", "packages/rustlib"]);
}

#[test]
fn workspace_discovery_rejects_layout_paths_outside_project_root() {
    let dir = tempdir().unwrap();
    let project = dir.path().join("project");
    let outside_package = dir.path().join("outside/package");
    fs::create_dir_all(&outside_package).unwrap();
    fs::write(outside_package.join("package.json"), "{}").unwrap();
    fs::create_dir_all(&project).unwrap();
    fs::write(
        project.join("magicore.workspace.toml"),
        "mode = \"monorepo\"\n[layout]\napps_dir = \"../outside\"\npackages_dir = \"packages\"\n",
    )
    .unwrap();

    let error = discover_workspace_projects(&project)
        .expect_err("workspace layout must not escape the project root");
    assert!(error.to_string().contains("workspace layout path"));
}

#[cfg(unix)]
#[test]
fn workspace_discovery_rejects_symlinked_layout_directory() {
    let dir = tempdir().unwrap();
    let project = dir.path().join("project");
    let outside_package = dir.path().join("outside/package");
    fs::create_dir_all(&outside_package).unwrap();
    fs::write(outside_package.join("package.json"), "{}").unwrap();
    fs::create_dir_all(&project).unwrap();
    std::os::unix::fs::symlink(dir.path().join("outside"), project.join("apps")).unwrap();
    fs::write(
        project.join("magicore.workspace.toml"),
        "mode = \"monorepo\"\n[layout]\napps_dir = \"apps\"\npackages_dir = \"packages\"\n",
    )
    .unwrap();

    let error = discover_workspace_projects(&project)
        .expect_err("workspace discovery must not follow a symlinked layout directory");
    assert!(error.to_string().contains("symlink"));
}

#[cfg(unix)]
#[test]
fn workspace_discovery_rejects_symlinked_workspace_config() {
    let dir = tempdir().unwrap();
    let project = dir.path().join("project");
    fs::create_dir_all(&project).unwrap();
    let external_config = dir.path().join("external-workspace.toml");
    fs::write(&external_config, "mode = \"monorepo\"\n").unwrap();
    std::os::unix::fs::symlink(external_config, project.join("magicore.workspace.toml")).unwrap();

    let error = discover_workspace_projects(&project)
        .expect_err("workspace config symlink must not be followed");
    assert!(error.to_string().contains("symlink"));
}

#[test]
fn workspace_discovery_propagates_invalid_core_marker_instead_of_descending() {
    let dir = tempdir().unwrap();
    let invalid_project = dir.path().join("apps/invalid");
    let nested_project = invalid_project.join("nested");
    fs::create_dir_all(&nested_project).unwrap();
    fs::write(invalid_project.join(".mgc.core"), "not-a-core\n").unwrap();
    fs::write(nested_project.join("package.json"), "{}").unwrap();

    let mut discovered = Vec::new();
    let error = collect_installable_projects(dir.path().join("apps"), &mut discovered)
        .expect_err("an invalid core marker must abort workspace discovery");

    assert!(
        error.to_string().contains("invalid core marker")
            && !error.to_string().contains("signature")
    );
    assert!(
        discovered.is_empty(),
        "nested projects must not bypass invalid parent identity"
    );
}

#[test]
fn test_discover_workspace_projects_ignores_non_monorepo_file() {
    let dir = tempdir().unwrap();
    fs::write(
        dir.path().join("magicore.workspace.toml"),
        r#"
version = 1
mode = "single"
"#,
    )
    .unwrap();

    assert!(discover_workspace_projects(dir.path()).unwrap().is_none());
}
