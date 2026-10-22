#![allow(clippy::unwrap_used)]
//! Installed dependency-root enumeration tests.
//! Test liệt kê root dependency đã cài.

use std::fs;

use tempfile::tempdir;

use super::super::allowed_dependency_roots;

#[test]
fn includes_ordinary_package_and_scoped_package_without_granting_parent_root() {
    let project = tempdir().unwrap();
    let modules = project.path().join("node_modules");
    fs::create_dir_all(modules.join("sample")).unwrap();
    fs::create_dir_all(modules.join("@scope/package")).unwrap();

    let roots = allowed_dependency_roots(&modules).unwrap();
    let expected = [
        fs::canonicalize(modules.join("sample")).unwrap(),
        fs::canonicalize(modules.join("@scope/package")).unwrap(),
    ];

    assert_eq!(roots.len(), expected.len());
    assert!(!roots.contains(&fs::canonicalize(&modules).unwrap()));
    for root in expected {
        assert!(roots.contains(&root));
    }
}

#[test]
fn includes_only_known_package_manager_store_roots() {
    let project = tempdir().unwrap();
    let modules = project.path().join("node_modules");
    fs::create_dir_all(modules.join(".pnpm/package@1.0.0/node_modules/package")).unwrap();
    fs::create_dir_all(modules.join(".cache/secret")).unwrap();

    let roots = allowed_dependency_roots(&modules).unwrap();

    assert!(roots.contains(&fs::canonicalize(modules.join(".pnpm")).unwrap()));
    assert!(!roots.contains(&fs::canonicalize(modules.join(".cache")).unwrap()));
}

#[cfg(unix)]
#[test]
fn includes_the_canonical_target_of_a_direct_workspace_package_link() {
    use std::os::unix::fs::symlink;

    let project = tempdir().unwrap();
    let modules = project.path().join("node_modules");
    let workspace_package = project.path().join("packages/shared");
    fs::create_dir_all(&modules).unwrap();
    fs::create_dir_all(&workspace_package).unwrap();
    symlink(&workspace_package, modules.join("shared")).unwrap();

    let roots = allowed_dependency_roots(&modules).unwrap();

    assert!(roots.contains(&fs::canonicalize(workspace_package).unwrap()));
}

#[cfg(unix)]
#[test]
fn includes_nested_workspace_dependency_links() {
    use std::os::unix::fs::symlink;

    let project = tempdir().unwrap();
    let modules = project.path().join("node_modules");
    let app = project.path().join("packages/app");
    let shared = project.path().join("packages/shared");
    fs::create_dir_all(&modules).unwrap();
    fs::create_dir_all(app.join("node_modules")).unwrap();
    fs::create_dir_all(&shared).unwrap();
    symlink(&app, modules.join("app")).unwrap();
    symlink(&shared, app.join("node_modules/shared")).unwrap();

    let roots = allowed_dependency_roots(&modules).unwrap();

    assert!(roots.contains(&fs::canonicalize(app).unwrap()));
    assert!(roots.contains(&fs::canonicalize(shared).unwrap()));
}

#[test]
fn rejects_missing_node_modules_root() {
    let project = tempdir().unwrap();

    assert!(allowed_dependency_roots(&project.path().join("missing")).is_err());
}
