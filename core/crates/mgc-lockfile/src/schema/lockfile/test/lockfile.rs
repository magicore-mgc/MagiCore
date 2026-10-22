//! Covers v3 lockfile root initialization and optional sections.
//! Kiểm tra khởi tạo root lockfile v3 và các phần tùy chọn.

use super::super::Lockfile;

#[test]
fn initializes_empty_v3_roots_and_optional_sections() {
    let lockfile = Lockfile::new();

    assert_eq!(
        lockfile.version,
        super::super::super::LOCKFILE_SCHEMA_VERSION
    );
    assert!(lockfile.root_dependencies.is_empty());
    assert!(lockfile.root_dependencies_by_owner.is_empty());
    assert!(lockfile.packages.is_empty());
    assert!(lockfile.workspace.is_none());
    assert!(lockfile.optimizer_profile.is_none());
    assert!(lockfile.metadata.dependency_ownership.is_empty());
}
