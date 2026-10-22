use super::super::super::error::GameOptimizerError;
use super::super::implementation::{prepare_staging_directory, remove_staging_directory};

#[cfg(unix)]
#[test]
fn optimizer_staging_rejects_linked_magicore_directory_without_touching_target() {
    use std::os::unix::fs::symlink;

    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("project");
    let outside = dir.path().join("outside");
    std::fs::create_dir_all(&root).unwrap();
    std::fs::create_dir_all(outside.join("tmp-optimizer")).unwrap();
    let sentinel = outside.join("tmp-optimizer").join("keep.txt");
    std::fs::write(&sentinel, "preserve outside data").unwrap();
    symlink(&outside, root.join(".magicore")).unwrap();

    let error = prepare_staging_directory(&root).unwrap_err();
    let typed = error
        .downcast_ref::<GameOptimizerError>()
        .expect("typed staging path error");
    assert!(typed.to_string().contains("linked optimizer staging path"));
    assert_eq!(
        std::fs::read_to_string(sentinel).unwrap(),
        "preserve outside data"
    );
}

#[cfg(unix)]
#[test]
fn optimizer_staging_rejects_linked_temp_directory_without_touching_target() {
    use std::os::unix::fs::symlink;

    let dir = tempfile::tempdir().unwrap();
    let root = dir.path().join("project");
    let outside = dir.path().join("outside");
    std::fs::create_dir_all(root.join(".magicore")).unwrap();
    std::fs::create_dir_all(&outside).unwrap();
    let sentinel = outside.join("keep.txt");
    std::fs::write(&sentinel, "preserve outside data").unwrap();
    symlink(&outside, root.join(".magicore").join("tmp-optimizer")).unwrap();

    let error = prepare_staging_directory(&root).unwrap_err();
    let typed = error
        .downcast_ref::<GameOptimizerError>()
        .expect("typed staging path error");
    assert!(typed.to_string().contains("linked optimizer staging path"));
    assert_eq!(
        std::fs::read_to_string(sentinel).unwrap(),
        "preserve outside data"
    );
}

#[test]
fn optimizer_staging_removes_stale_files_before_reuse() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    let stale = root
        .join(".magicore")
        .join("tmp-optimizer")
        .join("partial-template");
    std::fs::create_dir_all(stale.parent().unwrap()).unwrap();
    std::fs::write(&stale, "stale").unwrap();

    let staging = prepare_staging_directory(root).unwrap();

    assert_eq!(staging.path, root.join(".magicore").join("tmp-optimizer"));
    assert!(!stale.exists());
    assert!(staging.path.is_dir());
}

#[test]
fn optimizer_staging_removes_created_empty_magicore_directory() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();

    let staging = prepare_staging_directory(root).unwrap();
    assert!(root.join(".magicore").is_dir());
    remove_staging_directory(
        root,
        &staging.path,
        staging.remove_empty_parent,
        "test cleanup",
    )
    .unwrap();

    assert!(!root.join(".magicore").exists());
}
