//! Checks package-name validation at its owner.
//! Kiểm tra xác thực tên package tại owner gần nhất.

use super::super::PackageName;

#[test]
fn accepts_scoped_and_repository_identifiers() {
    assert_eq!(
        PackageName::new("@scope/pkg").unwrap().as_str(),
        "@scope/pkg"
    );
    assert_eq!(
        PackageName::new("github.com/owner/repo").unwrap().as_str(),
        "github.com/owner/repo"
    );
}

#[test]
fn rejects_traversal_and_empty_scopes() {
    assert!(PackageName::new("../secret").is_err());
    assert!(PackageName::new("@/pkg").is_err());
}
