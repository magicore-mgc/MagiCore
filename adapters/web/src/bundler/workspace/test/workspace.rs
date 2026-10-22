//! Tests for Web build workspace preparation.
//! Test chuẩn bị workspace build Web.

use super::super::prepare_workspace;
use tempfile::tempdir;

#[test]
fn prepare_workspace_links_node_modules_instead_of_mirroring_tree() {
    let tmp = tempdir().unwrap();
    let root = tmp.path();
    std::fs::create_dir_all(root.join("src")).unwrap();
    std::fs::create_dir_all(root.join("node_modules")).unwrap();
    std::fs::create_dir_all(root.join("node_modules/.magicore")).unwrap();
    std::fs::create_dir_all(root.join(".magicore")).unwrap();
    std::fs::write(
        root.join("package.json"),
        r#"{"name":"demo","version":"1.0.0"}"#,
    )
    .unwrap();
    std::fs::write(root.join("src/main.tsx"), "export const ok = true;").unwrap();

    let prepared = prepare_workspace(&root.join("src/main.tsx")).unwrap();

    assert_ne!(prepared.working_dir, root);
    assert!(prepared.entry.exists());
    let linked_node_modules = prepared.working_dir.join("node_modules");
    let metadata = std::fs::symlink_metadata(&linked_node_modules).unwrap();
    assert!(metadata.file_type().is_symlink() || metadata.is_dir());
    let mirror_path = prepared.working_dir.clone();
    drop(prepared);
    assert!(!mirror_path.exists());
}
