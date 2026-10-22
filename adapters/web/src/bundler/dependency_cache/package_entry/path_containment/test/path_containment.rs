#![allow(clippy::unwrap_used)]
//! Package-path containment regression tests.
//! Test hồi quy containment path package.

use std::fs;

use tempfile::tempdir;

use super::super::super::super::{PackageEntryError, resolve_package_entry};

#[test]
fn rejects_missing_manifest_targets_that_lexically_escape_package_root() {
    let root = tempdir().unwrap();
    let package = root.path().join("node_modules/sample");
    fs::create_dir_all(&package).unwrap();
    fs::write(package.join("index.js"), "safe fallback").unwrap();
    fs::write(
        package.join("package.json"),
        r#"{"main":"../../missing.js"}"#,
    )
    .unwrap();

    assert!(matches!(
        resolve_package_entry(&root.path().join("node_modules"), "sample"),
        Err(PackageEntryError::ForbiddenPath)
    ));
}
