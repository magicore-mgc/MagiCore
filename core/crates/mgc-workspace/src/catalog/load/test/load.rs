//! Workspace catalog file loading tests.
//! Test nạp catalog từ file workspace.

#![allow(clippy::unwrap_used)]

use crate::catalog::load_workspace_catalogs;

#[test]
fn missing_workspace_config_has_no_catalogs() {
    let root = tempfile::tempdir().unwrap();

    let catalogs = load_workspace_catalogs(root.path()).unwrap();

    assert!(catalogs.is_none());
}

#[test]
fn loads_default_and_named_catalogs() {
    let root = tempfile::tempdir().unwrap();
    std::fs::write(
        root.path().join("magicore.workspace.toml"),
        "[catalogs.default]\nserde = \"1.0\"\n[catalogs.tools]\ncargo-nextest = \"0.9\"\n",
    )
    .unwrap();

    let catalogs = load_workspace_catalogs(root.path()).unwrap().unwrap();

    assert_eq!(
        catalogs.default.get("serde").map(String::as_str),
        Some("1.0")
    );
    assert_eq!(
        catalogs
            .named
            .get("tools")
            .and_then(|catalog| catalog.get("cargo-nextest"))
            .map(String::as_str),
        Some("0.9")
    );
}
