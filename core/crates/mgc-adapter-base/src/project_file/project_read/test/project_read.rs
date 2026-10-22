//! Unix dirfd-based project read regression.
//! Kiểm tra đọc project qua dirfd trên Unix.

use super::read_project_regular_text_dirfd;
use std::fs;
use std::path::Path;

#[test]
fn reads_a_nested_regular_file_below_the_open_root() {
    let root = tempfile::tempdir().unwrap();
    fs::create_dir(root.path().join("nested")).unwrap();
    fs::write(root.path().join("nested/settings.toml"), "safe = true\n").unwrap();

    assert_eq!(
        read_project_regular_text_dirfd(
            root.path(),
            Path::new("nested/settings.toml"),
            "settings.toml"
        )
        .unwrap(),
        "safe = true\n"
    );
}
