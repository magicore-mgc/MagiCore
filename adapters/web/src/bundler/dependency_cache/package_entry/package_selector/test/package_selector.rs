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

/// Regression (P0 blank screen, 2026-10-31): `/@magicore/deps/react-dom/client`
/// returned 404 because a subpath with no literal file/directory was never
/// retried as `<subpath>.js`. Pinned here against the same layout as
/// `react-dom` 17: a root-level `client.js` (or `server.js`).
/// (Regression (P0 màn hình trắng): subpath không có file/thư mục nguyên
/// văn chưa từng được thử lại dạng `<subpath>.js`. Ghim tại đây với đúng
/// layout `react-dom` 17: file `client.js`/`server.js` ở package root.)
#[test]
fn resolves_extensionless_subpath_to_root_level_file() {
    let root = tempdir().unwrap();
    let package = root.path().join("node_modules/react-dom");
    fs::create_dir_all(&package).unwrap();
    fs::write(
        package.join("package.json"),
        r#"{"name":"react-dom","main":"index.js"}"#,
    )
    .unwrap();
    fs::write(package.join("index.js"), "export default 1;").unwrap();
    fs::write(package.join("client.js"), "export const client = 1;").unwrap();

    let resolved = resolve_package_entry(&root.path().join("node_modules"), "react-dom/client")
        .unwrap()
        .unwrap();
    assert_eq!(resolved, package.join("client.js").canonicalize().unwrap());
}

/// A subpath declared only by the manifest `exports` map must resolve even
/// when the target has no file extension on disk.
/// (Subpath chỉ được khai báo bằng map `exports` phải resolve được cả khi
/// target không có đuôi file trên đĩa.)
#[test]
fn resolves_subpath_through_the_exports_map() {
    let root = tempdir().unwrap();
    let package = root.path().join("node_modules/with-exports");
    fs::create_dir_all(package.join("dist")).unwrap();
    fs::write(
        package.join("package.json"),
        r#"{"exports":{".":"./dist/index.js","./client":{"browser":"./dist/client.browser.js","import":"./dist/client.mjs"}}}"#,
    )
    .unwrap();
    fs::write(package.join("dist/index.js"), "export default 1;").unwrap();
    fs::write(
        package.join("dist/client.browser.js"),
        "export const c = 1;",
    )
    .unwrap();
    fs::write(package.join("dist/client.mjs"), "export const c = 2;").unwrap();

    let node_modules = root.path().join("node_modules");
    let resolved = resolve_package_entry(&node_modules, "with-exports/client")
        .unwrap()
        .unwrap();
    assert_eq!(
        resolved,
        package
            .join("dist/client.browser.js")
            .canonicalize()
            .unwrap()
    );

    let entry = resolve_package_entry(&node_modules, "with-exports")
        .unwrap()
        .unwrap();
    assert_eq!(entry, package.join("dist/index.js").canonicalize().unwrap());
}

/// A subpath directory without a manifest resolves through its `index.js`.
/// (Thư mục subpath không có manifest resolve qua `index.js`.)
#[test]
fn resolves_subpath_directory_to_its_index_file() {
    let root = tempdir().unwrap();
    let package = root.path().join("node_modules/sample");
    fs::create_dir_all(package.join("dist/feature")).unwrap();
    fs::write(package.join("index.js"), "export default 1;").unwrap();
    fs::write(package.join("dist/feature/index.js"), "export const f = 1;").unwrap();

    let resolved = resolve_package_entry(&root.path().join("node_modules"), "sample/dist/feature")
        .unwrap()
        .unwrap();
    assert_eq!(
        resolved,
        package
            .join("dist/feature/index.js")
            .canonicalize()
            .unwrap()
    );
}

/// A genuinely absent subpath stays a miss so the route answers 404.
/// (Subpath thật sự không tồn tại vẫn là miss để route trả 404.)
#[test]
fn absent_subpath_returns_none() {
    let root = tempdir().unwrap();
    let package = root.path().join("node_modules/react-dom");
    fs::create_dir_all(&package).unwrap();
    fs::write(
        package.join("package.json"),
        r#"{"main":"index.js","exports":{".":"./index.js","./client":"./client.js"}}"#,
    )
    .unwrap();
    fs::write(package.join("index.js"), "export default 1;").unwrap();
    fs::write(package.join("client.js"), "export const client = 1;").unwrap();

    let node_modules = root.path().join("node_modules");
    for subpath in [
        "react-dom/absent",
        "react-dom/absent/index",
        "react-dom/server",
    ] {
        assert!(
            resolve_package_entry(&node_modules, subpath)
                .unwrap()
                .is_none(),
            "absent subpath {subpath:?} must not resolve"
        );
    }
}

/// `exports` must not resolve a target outside the package root.
/// (`exports` không được resolve target ra ngoài package root.)
#[test]
fn rejects_exports_targets_outside_the_package_root() {
    let root = tempdir().unwrap();
    let package = root.path().join("node_modules/sample");
    fs::create_dir_all(&package).unwrap();
    fs::write(root.path().join("outside.js"), "outside bytes").unwrap();
    fs::write(
        package.join("package.json"),
        r#"{"exports":{".":"./index.js","./client":"../outside.js"}}"#,
    )
    .unwrap();
    fs::write(package.join("index.js"), "export default 1;").unwrap();

    assert!(matches!(
        resolve_package_entry(&root.path().join("node_modules"), "sample/client"),
        Err(PackageEntryError::ForbiddenPath)
    ));
}
