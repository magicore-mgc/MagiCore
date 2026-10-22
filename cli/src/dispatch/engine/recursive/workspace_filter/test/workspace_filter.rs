#![allow(clippy::unwrap_used)]

use super::super::filter_workspaces;
use std::path::PathBuf;

#[test]
fn filters_by_scoped_manifest_name() {
    let root = tempfile::tempdir().unwrap();
    let api = root.path().join("apps/api");
    let web = root.path().join("apps/web");
    std::fs::create_dir_all(&api).unwrap();
    std::fs::create_dir_all(&web).unwrap();
    std::fs::write(api.join("package.json"), r#"{"name":"@core/api"}"#).unwrap();
    std::fs::write(web.join("package.json"), r#"{"name":"@other/web"}"#).unwrap();

    let selected = filter_workspaces(root.path(), &[api.clone(), web], "@core/*").unwrap();

    assert_eq!(selected, vec![api]);
}

#[test]
fn filters_by_relative_path_and_directory_name_fallback() {
    let root = tempfile::tempdir().unwrap();
    let workspace = root.path().join("apps/worker");
    std::fs::create_dir_all(&workspace).unwrap();

    let selected =
        filter_workspaces(root.path(), std::slice::from_ref(&workspace), "./apps/*").unwrap();

    assert_eq!(selected, vec![workspace]);
    let selected = filter_workspaces(root.path(), &selected, "worker").unwrap();
    assert_eq!(selected.len(), 1);
}

#[test]
fn malformed_manifest_error_is_not_treated_as_a_missing_name() {
    let root = tempfile::tempdir().unwrap();
    let workspace = root.path().join("apps/broken");
    std::fs::create_dir_all(&workspace).unwrap();
    std::fs::write(workspace.join("package.json"), "{").unwrap();

    let result = filter_workspaces(root.path(), &[PathBuf::from(&workspace)], "broken");

    assert!(result.is_err());
}
