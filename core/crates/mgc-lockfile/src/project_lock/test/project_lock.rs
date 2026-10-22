//! Pins the stable project guard path shape.
//! Ghim cấu trúc path guard project ổn định.

use super::super::ProjectWriteLock;
use std::path::{Path, PathBuf};

#[test]
fn guard_path_is_nested_under_magicore() {
    assert_eq!(
        ProjectWriteLock::guard_path_for(Path::new("project")),
        PathBuf::from("project")
            .join(".magicore")
            .join("lock.write")
    );
}
