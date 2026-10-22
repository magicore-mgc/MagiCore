//! Tests for safety-backup destination checks — kiểm tra đích safety backup.

use super::super::prepare_safety_backup;
use crate::commands::store::restore::backup_validation::ValidatedBackup;
use crate::commands::store::restore::project_state::inspect_project_state;
use std::fs;

#[test]
fn prepares_a_separate_explicit_safety_destination() {
    let temp = tempfile::tempdir().unwrap();
    let project = temp.path().join("project");
    fs::create_dir(&project).unwrap();
    let state = inspect_project_state(&project).unwrap();
    let backup_root = temp.path().join("backup");
    fs::create_dir(&backup_root).unwrap();
    let backup_root = fs::canonicalize(backup_root).unwrap();
    let backup = ValidatedBackup {
        root: backup_root,
        database: Some(temp.path().join("backup/store.db")),
        virtual_store: None,
    };
    let safety = temp.path().join("safety");

    let prepared = prepare_safety_backup(&backup, &state, Some(&safety)).unwrap();

    assert_eq!(prepared, fs::canonicalize(&safety).unwrap());
    assert!(prepared.is_dir());
}

#[test]
fn rejects_safety_destination_nested_under_the_restore_source() {
    let temp = tempfile::tempdir().unwrap();
    let project = temp.path().join("project");
    fs::create_dir(&project).unwrap();
    let state = inspect_project_state(&project).unwrap();
    let backup_root = temp.path().join("backup");
    fs::create_dir(&backup_root).unwrap();
    let backup_root = fs::canonicalize(backup_root).unwrap();
    let backup = ValidatedBackup {
        root: backup_root.clone(),
        database: Some(backup_root.join("store.db")),
        virtual_store: None,
    };
    let safety = backup_root.join("safety");

    let error = prepare_safety_backup(&backup, &state, Some(&safety)).unwrap_err();

    assert!(
        error
            .to_string()
            .contains("safety backup overlaps the selected restore source")
    );
    assert!(!safety.exists());
}

#[test]
fn rejects_safety_destination_inside_a_protected_project_store() {
    let temp = tempfile::tempdir().unwrap();
    let project = temp.path().join("project");
    fs::create_dir_all(project.join(".magicore/cache/web")).unwrap();
    let state = inspect_project_state(&project).unwrap();
    let backup_root = temp.path().join("backup");
    fs::create_dir(&backup_root).unwrap();
    let backup = ValidatedBackup {
        root: backup_root,
        database: Some(temp.path().join("backup/store.db")),
        virtual_store: None,
    };
    let safety = state.project_database_root.join("safety");

    let error = prepare_safety_backup(&backup, &state, Some(&safety)).unwrap_err();

    assert!(
        error
            .to_string()
            .contains("safety backup overlaps the selected restore source")
    );
    assert!(!safety.exists());
}
