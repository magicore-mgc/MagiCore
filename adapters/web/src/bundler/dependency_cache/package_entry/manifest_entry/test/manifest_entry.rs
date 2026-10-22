#![allow(clippy::unwrap_used)]
//! Manifest precedence and filesystem-boundary tests.
//! Test ưu tiên manifest và ranh giới filesystem.

use std::fs;

use tempfile::tempdir;

use super::super::super::super::{PackageEntryError, resolve_package_entry};

#[test]
fn prefers_browser_then_module_then_main() {
    for (manifest, expected) in [
        (
            r#"{"browser":"browser.js","module":"module.js","main":"main.js"}"#,
            "browser.js",
        ),
        (
            r#"{"browser":"missing.js","module":"module.js","main":"main.js"}"#,
            "module.js",
        ),
        (
            r#"{"browser":"missing.js","module":"missing.js","main":"main.js"}"#,
            "main.js",
        ),
    ] {
        let root = tempdir().unwrap();
        let package = root.path().join("node_modules/sample");
        fs::create_dir_all(&package).unwrap();
        for file in ["browser.js", "module.js", "main.js", "index.js"] {
            fs::write(package.join(file), "export default 1;").unwrap();
        }
        fs::write(package.join("package.json"), manifest).unwrap();

        let resolved = resolve_package_entry(&root.path().join("node_modules"), "sample")
            .unwrap()
            .unwrap();
        assert_eq!(resolved, package.join(expected).canonicalize().unwrap());
    }
}

#[test]
fn falls_back_to_index_when_entry_is_missing_or_manifest_is_malformed() {
    for manifest in [r#"{"main":"missing.js"}"#, "not-json"] {
        let root = tempdir().unwrap();
        let package = root.path().join("node_modules/sample");
        fs::create_dir_all(&package).unwrap();
        fs::write(package.join("package.json"), manifest).unwrap();
        fs::write(package.join("index.js"), "export default 1;").unwrap();

        let resolved = resolve_package_entry(&root.path().join("node_modules"), "sample")
            .unwrap()
            .unwrap();
        assert_eq!(resolved, package.join("index.js").canonicalize().unwrap());
    }
}

#[test]
fn accepts_dot_and_in_package_parent_entries() {
    for (package_path, manifest, expected) in [
        (
            "sample",
            r#"{"browser":"./dist/browser.js"}"#,
            "dist/browser.js",
        ),
        (
            "@scope/sample/subpath",
            r#"{"main":"../shared.js"}"#,
            "shared.js",
        ),
    ] {
        let root = tempdir().unwrap();
        let package_name = if package_path.starts_with('@') {
            "@scope/sample"
        } else {
            "sample"
        };
        let package = root.path().join("node_modules").join(package_name);
        fs::create_dir_all(package.join("dist")).unwrap();
        fs::create_dir_all(package.join("subpath")).unwrap();
        fs::write(package.join("dist/browser.js"), "export default 1;").unwrap();
        fs::write(package.join("shared.js"), "export default 1;").unwrap();
        let manifest_dir = if package_path.starts_with('@') {
            package.join("subpath")
        } else {
            package.clone()
        };
        fs::write(manifest_dir.join("package.json"), manifest).unwrap();

        let resolved = resolve_package_entry(&root.path().join("node_modules"), package_path)
            .unwrap()
            .unwrap();
        assert_eq!(resolved, package.join(expected).canonicalize().unwrap());
    }
}

#[test]
fn rejects_absolute_cross_platform_and_outside_manifest_entries() {
    let root = tempdir().unwrap();
    let package = root.path().join("node_modules/sample");
    fs::create_dir_all(&package).unwrap();

    for entry in [
        "/tmp/secret.js",
        "C:/Windows/secret.js",
        r"..\..\secret.js",
        "../../outside.js",
    ] {
        fs::write(
            package.join("package.json"),
            serde_json::json!({ "main": entry }).to_string(),
        )
        .unwrap();
        assert!(
            matches!(
                resolve_package_entry(&root.path().join("node_modules"), "sample"),
                Err(PackageEntryError::ForbiddenPath)
            ),
            "manifest entry {entry:?} must be rejected"
        );
    }
}

#[cfg(unix)]
#[test]
fn rejects_manifest_entry_symlinks_outside_package_root() {
    let root = tempdir().unwrap();
    let package = root.path().join("node_modules/sample");
    let outside = root.path().join("outside.js");
    fs::create_dir_all(&package).unwrap();
    fs::write(&outside, "outside bytes").unwrap();
    fs::write(package.join("package.json"), r#"{"main":"entry.js"}"#).unwrap();
    std::os::unix::fs::symlink(&outside, package.join("entry.js")).unwrap();

    assert!(matches!(
        resolve_package_entry(&root.path().join("node_modules"), "sample"),
        Err(PackageEntryError::ForbiddenPath)
    ));
}
