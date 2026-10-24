//! Archive safety: no entry may escape the scaffold workspace.
//! An toàn archive: không entry nào được thoát workspace scaffold.

use super::super::safe_relative_path;
use std::path::PathBuf;

#[test]
fn plain_relative_entries_are_accepted() {
    assert_eq!(
        safe_relative_path("src/main/java/App.java"),
        Some(PathBuf::from("src/main/java/App.java"))
    );
    assert_eq!(
        safe_relative_path("./src/app.ts"),
        Some(PathBuf::from("src/app.ts"))
    );
}

#[test]
fn parent_traversal_is_rejected() {
    for entry in [
        "../etc/passwd",
        "src/../../etc/passwd",
        "..\\windows\\system32",
        "src/../..",
    ] {
        assert_eq!(
            safe_relative_path(entry),
            None,
            "entry '{entry}' must be rejected"
        );
    }
}

#[test]
fn absolute_and_drive_paths_are_rejected() {
    for entry in [
        "/etc/passwd",
        "\\windows\\system32",
        "C:/windows",
        "//server/share",
    ] {
        assert_eq!(
            safe_relative_path(entry),
            None,
            "entry '{entry}' must be rejected"
        );
    }
}

#[test]
fn empty_entries_are_rejected() {
    assert_eq!(safe_relative_path(""), None);
    assert_eq!(safe_relative_path("."), None);
    assert_eq!(safe_relative_path("./"), None);
}
