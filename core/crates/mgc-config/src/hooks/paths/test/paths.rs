//! Tests for project hook configuration path selection.
//! Test chọn đường dẫn cấu hình hook dự án.

use super::super::hooks_paths;

#[test]
fn project_hook_config_is_the_first_path() {
    let root = tempfile::tempdir().unwrap();
    assert_eq!(
        hooks_paths(root.path())[0],
        root.path().join("mgc.hooks.toml")
    );
}
