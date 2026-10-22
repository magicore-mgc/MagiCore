//! Project-relative traversal rejection.
//! Kiểm tra từ chối path thoát khỏi project root.

#[cfg(not(unix))]
use super::checked_project_path;
#[cfg(unix)]
use super::project_parent_dir;
use std::path::Path;

#[test]
fn rejects_parent_components_before_resolving_a_file() {
    let root = tempfile::tempdir().unwrap();
    let relative = Path::new("../outside.toml");

    #[cfg(unix)]
    assert!(project_parent_dir(root.path(), relative).is_err());
    #[cfg(not(unix))]
    assert!(checked_project_path(root.path(), relative).is_err());
}
