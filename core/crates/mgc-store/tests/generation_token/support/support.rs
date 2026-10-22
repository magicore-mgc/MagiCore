//! Hermetic database fixture and project identifiers.
//! Fixture database cô lập và định danh project.

use mgc_store::Database;

pub(crate) struct TestDb {
    _dir: tempfile::TempDir,
    pub(crate) db: Database,
}

impl TestDb {
    pub(crate) fn new() -> Self {
        let dir = tempfile::tempdir().unwrap();
        let db = Database::open(&dir.path().join("store.db")).unwrap();
        Self { _dir: dir, db }
    }
}

/// Prove an orphan claim is rejected by SQLite's composite foreign key.
/// Xác nhận SQLite từ chối claim mồ côi bằng foreign key ghép.
pub(crate) fn assert_orphan_claim_rejected(db: &Database, project_root: &str, generation: i64) {
    let error = db
        .conn_for_tests()
        .execute(
            "INSERT INTO cas_blob_refs (project_root, generation, hash)
             VALUES (?1, ?2, 'hash-orphan')",
            rusqlite::params![project_root, generation],
        )
        .unwrap_err();

    assert_eq!(
        error.sqlite_error().map(|sqlite| sqlite.extended_code),
        Some(rusqlite::ffi::SQLITE_CONSTRAINT_FOREIGNKEY),
        "orphan CAS claim must fail specifically at the SQLite foreign-key constraint: {error}"
    );
    let orphan_count: i64 = db
        .conn_for_tests()
        .query_row(
            "SELECT COUNT(*) FROM cas_blob_refs
             WHERE project_root = ?1 AND generation = ?2 AND hash = 'hash-orphan'",
            rusqlite::params![project_root, generation],
            |row| row.get(0),
        )
        .unwrap();
    assert_eq!(orphan_count, 0, "rejected orphan claim must not persist");
}

pub(crate) const PROJ_A: &str = "/proj/a";
pub(crate) const PROJ_B: &str = "/proj/b";
