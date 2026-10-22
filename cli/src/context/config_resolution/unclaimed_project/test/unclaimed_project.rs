//! Verify explicit core selection preserves synthetic project naming.
//! Kiểm tra lựa chọn core tường minh giữ quy tắc tên project tạm.

use std::path::Path;

use super::*;

#[test]
fn existing_root_uses_its_directory_name() {
    let root = Path::new("/workspace/api");
    let config = for_project_root(root, "web");

    assert_eq!(config.name, "api");
    assert_eq!(config.ecosystem, "web");
}

#[test]
fn missing_root_uses_the_established_synthetic_name() {
    let config = without_project_root("ai");

    assert_eq!(config.name, "project");
    assert_eq!(config.ecosystem, "ai");
}
