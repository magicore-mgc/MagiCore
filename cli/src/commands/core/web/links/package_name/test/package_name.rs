//! Tests for the existing package-link name rule.
//! Test quy tắc tên link package hiện tại.

use super::super::implementation::link_name;

#[test]
fn scoped_package_link_name_preserves_scope_path() {
    let name = link_name("@scope/package").expect("valid scoped package");
    let expected = std::path::Path::new("@scope").join("package");

    assert_eq!(name.target_path(), expected.as_path());
    assert_eq!(name.display_name(), "@scope/package");
}

#[test]
fn plain_package_link_name_keeps_its_leaf_target() {
    let name = link_name("plain-package").expect("valid package");

    assert_eq!(name.target_path().to_str(), Some("plain-package"));
    assert_eq!(name.display_name(), "plain-package");
}

#[test]
fn explicit_relative_path_uses_its_final_path_segment() {
    let name = link_name("../local-package").expect("valid relative path");

    assert_eq!(name.target_path().to_str(), Some("local-package"));
}

#[test]
fn rejects_scoped_names_that_escape_or_create_platform_special_paths() {
    for package in [
        "@scope",
        "@scope/../outside",
        "@scope/package/extra",
        "@scope/pkg:stream",
        "@scope/pkg.",
        "@scope/pkg ",
    ] {
        assert!(link_name(package).is_err(), "accepted {package:?}");
    }
}

#[test]
fn rejects_unscoped_package_path_traversal() {
    for package in ["nested/../outside", "nested//package", "nested\\package"] {
        assert!(link_name(package).is_err(), "accepted {package:?}");
    }
}
