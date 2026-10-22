//! Workspace layout configuration contract tests.
//! Test hợp đồng cho cấu hình layout workspace.

#![allow(clippy::unwrap_used)]

use crate::discover_workspace_targets;

#[test]
fn discover_respects_custom_layout() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    std::fs::create_dir_all(root.join("src/apps/x")).unwrap();
    std::fs::create_dir_all(root.join("libs/y")).unwrap();
    std::fs::write(root.join("src/apps/x/package.json"), "{}").unwrap();
    std::fs::write(root.join("libs/y/package.json"), "{}").unwrap();
    std::fs::write(
        root.join("magicore.workspace.toml"),
        "[layout]\napps_dir = \"src/apps\"\npackages_dir = \"libs\"\n",
    )
    .unwrap();

    let targets = discover_workspace_targets(root).unwrap();
    let names: Vec<String> = targets
        .iter()
        .map(|t| t.strip_prefix(root).unwrap().to_string_lossy().to_string())
        .collect::<Vec<_>>()
        .into_iter()
        .map(|s| s.replace("src/apps", "apps"))
        .collect();
    assert!(names.iter().any(|n| n.ends_with("x")));
    assert!(names.iter().any(|n| n.ends_with("y")));
}

#[cfg(unix)]
#[test]
fn read_workspace_config_rejects_a_symlinked_file() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path().join("project");
    let outside = tmp.path().join("outside.toml");
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(&outside, "[layout]\napps_dir = \"outside\"\n").unwrap();
    std::os::unix::fs::symlink(&outside, root.join("magicore.workspace.toml")).unwrap();

    let error = crate::read_workspace_config(&root).unwrap_err();

    assert!(error.to_string().contains("symlink"));
}
