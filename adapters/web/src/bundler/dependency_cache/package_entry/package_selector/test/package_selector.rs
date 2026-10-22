#![allow(clippy::unwrap_used)]
//! Package splitting and subpath-containment tests.
//! Test tách package và containment subpath.

use std::fs;

use tempfile::tempdir;

use super::super::super::super::{PackageEntryError, resolve_package_entry};

#[test]
fn resolves_unscoped_package_subpath_files_directly() {
    let root = tempdir().unwrap();
    let package = root.path().join("node_modules/lodash");
    fs::create_dir_all(package.join("fp")).unwrap();
    fs::write(package.join("fp/map.js"), "export default 1;").unwrap();

    let resolved = resolve_package_entry(&root.path().join("node_modules"), "lodash/fp/map.js")
        .unwrap()
        .unwrap();
    assert_eq!(resolved, package.join("fp/map.js").canonicalize().unwrap());
}

#[cfg(unix)]
#[test]
fn rejects_package_subpath_symlinks_outside_package_root() {
    let root = tempdir().unwrap();
    let package = root.path().join("node_modules/sample");
    let outside = root.path().join("outside.js");
    fs::create_dir_all(&package).unwrap();
    fs::write(package.join("index.js"), "export default 1;").unwrap();
    fs::write(&outside, "outside bytes").unwrap();
    std::os::unix::fs::symlink(&outside, package.join("outside.js")).unwrap();

    assert!(matches!(
        resolve_package_entry(&root.path().join("node_modules"), "sample/outside.js"),
        Err(PackageEntryError::ForbiddenPath)
    ));
}

#[cfg(unix)]
#[test]
fn allows_pnpm_and_workspace_package_root_symlinks() {
    for use_workspace in [false, true] {
        let root = tempdir().unwrap();
        let node_modules = if use_workspace {
            root.path().join("apps/site/node_modules")
        } else {
            root.path().join("node_modules")
        };
        let target = if use_workspace {
            root.path().join("packages/sample")
        } else {
            node_modules.join(".pnpm/sample@1.0.0/node_modules/sample")
        };
        fs::create_dir_all(&target).unwrap();
        fs::write(target.join("package.json"), r#"{"main":"./index.js"}"#).unwrap();
        fs::write(target.join("index.js"), "export default 1;").unwrap();
        fs::create_dir_all(&node_modules).unwrap();
        std::os::unix::fs::symlink(&target, node_modules.join("sample")).unwrap();

        let resolved = resolve_package_entry(&node_modules, "sample")
            .unwrap()
            .unwrap();
        assert_eq!(resolved, target.join("index.js").canonicalize().unwrap());
    }
}
