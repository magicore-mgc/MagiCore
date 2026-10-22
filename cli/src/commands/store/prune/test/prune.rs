//! Prune tests keep dry-run, deletion, missing-database and lock outcomes explicit.
//! Test prune ghi rõ kết quả dry-run, xóa, DB thiếu và xung đột lock.

use super::super::prune_unreferenced;
use mgc_store::{Database, Layout, ProjectInstallLock};
use mgc_types::PackageId;
use std::fs;

fn seed_unreferenced_package(project_root: &std::path::Path) -> (PackageId, std::path::PathBuf) {
    let package = PackageId::parse("react@18.2.0").unwrap();
    let cache_root = project_root.join(".magicore/cache/web");
    fs::create_dir_all(&cache_root).unwrap();
    let database = Database::open(&cache_root.join("store.db")).unwrap();
    database
        .insert_package(&package, Some("sha512-test"))
        .unwrap();

    let package_dir = project_root
        .join("node_modules/.magicore")
        .join("react@18.2.0");
    fs::create_dir_all(&package_dir).unwrap();
    fs::write(package_dir.join("payload.bin"), b"payload").unwrap();
    (package, package_dir)
}

#[test]
fn missing_database_is_a_noop_and_does_not_create_lock_state() {
    let temp = tempfile::tempdir().unwrap();
    fs::create_dir_all(temp.path().join(".magicore/cache/web")).unwrap();

    let report = prune_unreferenced(temp.path(), false).unwrap();

    assert!(report.unreferenced.is_empty());
    assert!(report.removed.is_empty());
    assert_eq!(report.removed_bytes, 0);
    assert!(!temp.path().join(".magicore/cache/web/locks").exists());
    assert!(!temp.path().join(".magicore/cache/web/store.db").exists());
}

#[test]
fn dry_run_reports_projected_logical_bytes_without_deleting_package_or_row() {
    let temp = tempfile::tempdir().unwrap();
    let (package, package_dir) = seed_unreferenced_package(temp.path());

    let report = prune_unreferenced(temp.path(), true).unwrap();

    assert!(report.dry_run);
    assert_eq!(report.unreferenced, vec![package.to_string()]);
    assert_eq!(report.removed, vec![package.to_string()]);
    assert_eq!(report.removed_bytes, 7);
    assert!(package_dir.join("payload.bin").is_file());
    let database = Database::open(&temp.path().join(".magicore/cache/web/store.db")).unwrap();
    assert!(database.is_installed(&package).unwrap());
}

#[test]
fn real_prune_removes_package_payload_and_database_record() {
    let temp = tempfile::tempdir().unwrap();
    let (package, package_dir) = seed_unreferenced_package(temp.path());

    let report = prune_unreferenced(temp.path(), false).unwrap();

    assert!(!report.dry_run);
    assert_eq!(report.removed, vec![package.to_string()]);
    assert_eq!(report.removed_bytes, 7);
    assert!(!package_dir.exists());
    let database = Database::open(&temp.path().join(".magicore/cache/web/store.db")).unwrap();
    assert!(!database.is_installed(&package).unwrap());
}

#[test]
fn active_web_project_lock_blocks_prune_before_mutation() {
    let temp = tempfile::tempdir().unwrap();
    let (package, package_dir) = seed_unreferenced_package(temp.path());
    let cache_root = temp.path().join(".magicore/cache/web");
    let layout = Layout::new(cache_root);
    let _web_install_lock =
        ProjectInstallLock::acquire_at(&layout.locks_dir(), &layout.root().to_string_lossy())
            .unwrap();

    assert!(prune_unreferenced(temp.path(), false).is_err());

    assert!(package_dir.join("payload.bin").is_file());
    let database = Database::open(&temp.path().join(".magicore/cache/web/store.db")).unwrap();
    assert!(database.is_installed(&package).unwrap());
}

#[cfg(any(unix, windows))]
#[test]
fn unsafe_vstore_root_is_rejected_before_database_migration_or_lock_creation() {
    let temp = tempfile::tempdir().unwrap();
    let cache_root = temp.path().join(".magicore/cache/web");
    fs::create_dir_all(&cache_root).unwrap();
    let database_path = cache_root.join("store.db");
    fs::write(&database_path, b"").unwrap();
    let external = temp.path().join("external-vstore");
    fs::create_dir(&external).unwrap();
    let node_modules = temp.path().join("node_modules");
    fs::create_dir(&node_modules).unwrap();
    super::symlink_support::symlink_directory(&external, &node_modules.join(".magicore")).unwrap();

    assert!(prune_unreferenced(temp.path(), false).is_err());

    assert_eq!(fs::metadata(&database_path).unwrap().len(), 0);
    assert!(!cache_root.join("locks").exists());
}

#[cfg(any(unix, windows))]
#[test]
fn symlink_candidate_aborts_prune_and_preserves_external_target() {
    let temp = tempfile::tempdir().unwrap();
    let (package, package_dir) = seed_unreferenced_package(temp.path());
    fs::remove_dir_all(&package_dir).unwrap();
    let external = temp.path().join("external");
    fs::create_dir(&external).unwrap();
    fs::write(external.join("payload.bin"), b"external").unwrap();
    super::symlink_support::symlink_directory(
        &external,
        &temp.path().join("node_modules/.magicore/react@18.2.0"),
    )
    .unwrap();

    assert!(prune_unreferenced(temp.path(), false).is_err());

    assert_eq!(fs::read(external.join("payload.bin")).unwrap(), b"external");
    let database = Database::open(&temp.path().join(".magicore/cache/web/store.db")).unwrap();
    assert!(database.is_installed(&package).unwrap());
}
