//! Tests for post-lock project state capture — kiểm tra chụp state sau lock.

use super::super::{inspect_project_state, validate_restore_source};
use crate::commands::store::restore::backup_validation::ValidatedBackup;
use std::fs;

#[test]
fn captures_canonical_project_and_empty_store_destinations() {
    let temp = tempfile::tempdir().unwrap();
    let project = temp.path().join("project");
    fs::create_dir(&project).unwrap();

    let state = inspect_project_state(&project).unwrap();

    assert_eq!(state.canonical_project, fs::canonicalize(&project).unwrap());
    assert!(state.current_database.is_none());
    assert!(state.current_vstore.is_none());
    assert_eq!(
        state.project_database_root,
        state.canonical_project.join(".magicore/cache/web")
    );
    assert_eq!(
        state.project_vstore_root,
        state.canonical_project.join("node_modules/.magicore")
    );
}

#[test]
fn rejects_a_restore_source_inside_a_protected_project_database_root() {
    let temp = tempfile::tempdir().unwrap();
    let project = temp.path().join("project");
    fs::create_dir(&project).unwrap();
    let state = inspect_project_state(&project).unwrap();
    let source = state.project_database_root.join("backup");
    let backup = ValidatedBackup {
        root: source.clone(),
        database: Some(source.join("store.db")),
        virtual_store: None,
    };

    let error = validate_restore_source(&backup, &state).unwrap_err();

    assert!(
        error
            .to_string()
            .contains("safety backup overlaps the selected restore source")
    );
}
