#![allow(clippy::unwrap_used)]
// filesystem owner tests — kiểm thử owner filesystem.
// These checks pin idempotent cleanup and reserved install-directory retention.
// Kiểm thử khóa hành vi dọn lặp an toàn và giữ nguyên thư mục dành riêng.
use super::*;
use crate::install::materialize::test::support::{package_id, tempdir_real};

#[test]
fn removal_is_idempotent_and_removes_directory_trees() {
    let temp = tempdir_real();
    let directory = temp.path().join("node_modules/stale");
    std::fs::create_dir_all(&directory).unwrap();
    std::fs::write(directory.join("index.js"), "module.exports = 1;").unwrap();

    remove_fs_entry(&directory).unwrap();
    remove_fs_entry(&directory).unwrap();

    assert!(!directory.exists());
}

#[test]
fn root_pruning_preserves_reserved_dirs_and_expected_scoped_packages() {
    let temp = tempdir_real();
    let modules = temp.path().join("node_modules");
    for relative in [
        ".bin/tool",
        ".magicore/state",
        "react/package.json",
        "stale/package.json",
        "@scope/keep/package.json",
        "@scope/drop/package.json",
    ] {
        let path = modules.join(relative);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, "{}").unwrap();
    }
    let expected = std::collections::HashMap::from([
        ("react".to_string(), package_id("react@19.0.0")),
        ("@scope/keep".to_string(), package_id("@scope/keep@1.0.0")),
    ]);

    prune_root_install_dirs(&modules, &expected).unwrap();

    assert!(modules.join(".bin/tool").exists());
    assert!(modules.join(".magicore/state").exists());
    assert!(modules.join("react/package.json").exists());
    assert!(!modules.join("stale").exists());
    assert!(modules.join("@scope/keep/package.json").exists());
    assert!(!modules.join("@scope/drop").exists());
}
