//! Workspace layout containment and symlink tests.
//! Test giới hạn layout workspace và từ chối symlink.

#![allow(clippy::unwrap_used)]

use crate::discover_workspace_targets;

#[test]
fn discover_rejects_layout_paths_that_escape_the_project_root() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path().join("project");
    let outside = tmp.path().join("outside");
    std::fs::create_dir_all(outside.join("pkg")).unwrap();
    std::fs::write(outside.join("pkg/package.json"), "{}").unwrap();
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(
        root.join("magicore.workspace.toml"),
        "[layout]\napps_dir = \"../outside\"\npackages_dir = \"packages\"\n",
    )
    .unwrap();

    let error = discover_workspace_targets(&root).unwrap_err();

    assert!(error.to_string().contains("workspace layout path"));
}

#[cfg(unix)]
#[test]
fn discover_rejects_symlinked_workspace_directories() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path().join("project");
    let outside = tmp.path().join("outside");
    std::fs::create_dir_all(outside.join("pkg")).unwrap();
    std::fs::write(outside.join("pkg/package.json"), "{}").unwrap();
    std::fs::create_dir_all(&root).unwrap();
    std::os::unix::fs::symlink(&outside, root.join("apps")).unwrap();

    let error = discover_workspace_targets(&root).unwrap_err();

    assert!(error.to_string().contains("symlink"));
}
