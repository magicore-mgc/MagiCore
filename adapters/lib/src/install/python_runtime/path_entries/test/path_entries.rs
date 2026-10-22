//! Python path-entry behavior and absent-lock tests.
//! Test behavior đường dẫn và lock không tồn tại.

use super::support::*;

#[test]
fn python_runtime_paths_fail_closed_on_malformed_lock_and_ignore_absent_lock() {
    let dir = tempfile::tempdir().unwrap();
    assert!(native_python_path_entries(dir.path()).unwrap().is_empty());

    let path = dir.path().join("mgc.lock");
    std::fs::write(&path, "not valid lock TOML [[[").unwrap();
    assert!(
        native_python_path_entries(dir.path())
            .unwrap_err()
            .to_string()
            .contains("cannot safely read mgc.lock for Python runtime")
    );
}
