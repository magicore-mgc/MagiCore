//! Public project-file API round-trip regression.
//! Kiểm tra vòng đọc/ghi qua API public của project-file.

use super::{atomic_write_project_regular, read_project_regular_text};
use std::fs;
use std::path::Path;

#[test]
fn project_regular_read_and_write_round_trip() {
    let root = tempfile::tempdir().unwrap();
    let relative = Path::new("project.toml");
    fs::write(root.path().join(relative), "name = 'before'\n").unwrap();

    assert_eq!(
        read_project_regular_text(root.path(), relative, "project.toml").unwrap(),
        "name = 'before'\n"
    );
    atomic_write_project_regular(root.path(), relative, b"name = 'after'\n").unwrap();
    assert_eq!(
        read_project_regular_text(root.path(), relative, "project.toml").unwrap(),
        "name = 'after'\n"
    );
}
