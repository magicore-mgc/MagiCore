//! SQLite integer boundary regression tests for store metadata.
//! Kiểm tra hồi quy giới hạn số nguyên SQLite cho metadata của store.

#![allow(clippy::unwrap_used)]

use mgc_store::{Database, Layout, StoreIndex};

#[test]
fn unsigned_metadata_roundtrips_and_rejects_unrepresentable_values() {
    let dir = tempfile::tempdir().unwrap();
    let db = Database::open(&dir.path().join("store.db")).unwrap();

    db.upsert_release_policy("npm", i64::MAX as u64).unwrap();
    assert_eq!(db.release_policy("npm").unwrap(), Some(i64::MAX as u64));
    assert!(db.upsert_release_policy("bun", u64::MAX).is_err());
    assert_eq!(db.release_policy("bun").unwrap(), None);

    let id = mgc_types::PackageId::parse("probe@1.0.0").unwrap();
    db.insert_package(&id, Some("sha512-probe")).unwrap();
    assert!(db.list_installed().unwrap()[0].installed_at > 0);
    db.conn()
        .execute("UPDATE packages SET installed_at = -1", [])
        .unwrap();
    assert!(db.list_installed().is_err());

    db.upsert_trust_policy("probe", "allow").unwrap();
    db.conn()
        .execute("UPDATE trust_policy SET updated_at = -1", [])
        .unwrap();
    assert!(db.list_trust_policies().is_err());

    db.conn()
        .execute("UPDATE release_policy SET min_age_secs = -1", [])
        .unwrap();
    assert!(db.release_policy("npm").is_err());
}

#[test]
fn package_file_overflow_rolls_back_replacement() {
    let dir = tempfile::tempdir().unwrap();
    let db = Database::open(&dir.path().join("store.db")).unwrap();
    let id = mgc_types::PackageId::parse("probe@1.0.0").unwrap();
    let original = vec![("lib.rs".to_string(), "hash-a".to_string(), 7)];
    db.replace_package_files(&id, &original).unwrap();

    let overflow = vec![("lib.rs".to_string(), "hash-b".to_string(), u64::MAX)];
    assert!(db.replace_package_files(&id, &overflow).is_err());
    assert_eq!(db.list_package_files(&id).unwrap(), original);
}

#[test]
fn negative_package_file_size_is_rejected_by_database_and_index_rebuild() {
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("store.db");
    let id = mgc_types::PackageId::parse("probe@1.0.0").unwrap();
    {
        let db = Database::open(&db_path).unwrap();
        db.replace_package_files(&id, &[("lib.rs".to_string(), "hash-a".to_string(), 7)])
            .unwrap();
        db.conn()
            .execute("UPDATE package_files SET size = -1", [])
            .unwrap();
        assert!(db.list_package_files(&id).is_err());
    }

    let layout = Layout::new(dir.path().to_path_buf());
    assert!(StoreIndex::open(&layout, Database::open(&db_path).unwrap()).is_err());
}
