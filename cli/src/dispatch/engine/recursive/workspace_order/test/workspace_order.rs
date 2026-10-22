#![allow(clippy::unwrap_used)]

use super::super::order_workspaces;
use std::path::PathBuf;

#[test]
fn orders_dependency_manifests_before_dependents() {
    let root = tempfile::tempdir().unwrap();
    let app = root.path().join("app");
    let library = root.path().join("library");
    std::fs::create_dir_all(&app).unwrap();
    std::fs::create_dir_all(&library).unwrap();
    std::fs::write(
        app.join("package.json"),
        r#"{"name":"app","dependencies":{"library":"workspace:*"}}"#,
    )
    .unwrap();
    std::fs::write(library.join("package.json"), r#"{"name":"library"}"#).unwrap();

    let ordered = order_workspaces(vec![app.clone(), library.clone()]).unwrap();

    assert_eq!(ordered, vec![library, app]);
}

#[test]
fn keeps_manifestless_paths_in_input_order() {
    let root = tempfile::tempdir().unwrap();
    let first = root.path().join("first");
    let second = root.path().join("second");
    std::fs::create_dir_all(&first).unwrap();
    std::fs::create_dir_all(&second).unwrap();

    let ordered = order_workspaces(vec![first.clone(), second.clone()]).unwrap();

    assert_eq!(ordered, vec![first, second]);
}

#[test]
fn returns_graph_errors_instead_of_falling_back_to_input_order() {
    let root = tempfile::tempdir().unwrap();
    let app = root.path().join("app");
    std::fs::create_dir_all(&app).unwrap();
    std::fs::write(
        app.join("package.json"),
        r#"{"name":"app","dependencies":{"missing":"workspace:*"}}"#,
    )
    .unwrap();

    let result = order_workspaces(vec![PathBuf::from(&app)]);

    assert!(result.is_err());
}

#[test]
fn rejects_workspace_dependency_cycles() {
    let root = tempfile::tempdir().unwrap();
    let app = root.path().join("app");
    let library = root.path().join("library");
    std::fs::create_dir_all(&app).unwrap();
    std::fs::create_dir_all(&library).unwrap();
    std::fs::write(
        app.join("package.json"),
        r#"{"name":"app","dependencies":{"library":"workspace:*"}}"#,
    )
    .unwrap();
    std::fs::write(
        library.join("package.json"),
        r#"{"name":"library","dependencies":{"app":"workspace:*"}}"#,
    )
    .unwrap();

    let result = order_workspaces(vec![app, library]);

    assert!(result.is_err());
}
