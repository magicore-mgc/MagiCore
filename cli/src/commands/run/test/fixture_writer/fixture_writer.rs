//! Write temporary project fixture files.
//! Ghi file fixture project tạm cho test.

use std::path::PathBuf;

pub(in crate::commands::run::tests) fn write(path: PathBuf, content: &str) {
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, content).unwrap();
}
