//! Legacy v2 store migration and claim preservation.
//! Migration store v2 cũ và bảo toàn claim.

use crate::generation_token::support::assert_orphan_claim_rejected;
use mgc_store::Database;

pub(crate) fn migration_v2_shape_preserves_claims_across_api_promote() {
    // Build a v2-shaped store, migrate it, verify the FK rejects an orphan,
    // then ensure existing claims survive API claim and promote operations.
    // Dựng và migrate store dạng v2, xác nhận FK từ chối claim mồ côi,
    // sau đó kiểm tra claim cũ sống qua API claim và promote.
    let dir = tempfile::tempdir().unwrap();
    let db_path = dir.path().join("store.db");
    {
        let conn = rusqlite::Connection::open(&db_path).unwrap();
        conn.execute_batch(
            "CREATE TABLE packages (
                id TEXT NOT NULL, version TEXT NOT NULL, integrity TEXT,
                installed_at INTEGER NOT NULL, PRIMARY KEY (id, version));
             CREATE TABLE integrity_cache (hash TEXT PRIMARY KEY, verified_at INTEGER NOT NULL);
             CREATE TABLE refs (project_root TEXT NOT NULL, package_id TEXT NOT NULL,
                ref_count INTEGER NOT NULL DEFAULT 0, PRIMARY KEY (project_root, package_id));
             CREATE TABLE cas_generations (
                project_root TEXT NOT NULL, generation INTEGER NOT NULL,
                state TEXT NOT NULL DEFAULT 'staging'
                    CHECK (state IN ('staging','promoted')),
                PRIMARY KEY (project_root, generation));
             CREATE TABLE cas_blob_refs (
                project_root TEXT NOT NULL, generation INTEGER NOT NULL DEFAULT 0,
                hash TEXT NOT NULL, PRIMARY KEY (project_root, generation, hash));
             CREATE TABLE package_files (id TEXT NOT NULL, version TEXT NOT NULL,
                path TEXT NOT NULL, blob_hash TEXT NOT NULL, size INTEGER NOT NULL DEFAULT 0,
                PRIMARY KEY (id, version, path));
             CREATE TABLE trust_policy (package_id TEXT PRIMARY KEY, policy TEXT NOT NULL,
                updated_at INTEGER NOT NULL);
             CREATE TABLE release_policy (ecosystem TEXT PRIMARY KEY, min_age_secs INTEGER NOT NULL);
             INSERT INTO cas_generations VALUES ('/proj/legacy', 1, 'staging');
             INSERT INTO cas_blob_refs VALUES ('/proj/legacy', 1, 'hash-old');
             PRAGMA journal_mode=WAL;",
        )
        .unwrap();
    }

    let db = Database::open(&db_path).unwrap();
    let live = db.list_cas_live_refs().unwrap();
    assert_eq!(live, vec!["hash-old".to_string()]);
    assert_orphan_claim_rejected(&db, "/proj/legacy", 99);

    // The migrated staging marker remains usable through the API.
    // Marker staging sau migration vẫn dùng được qua API.
    db.cas_claim("/proj/legacy", 1, "hash-old").unwrap();
    db.cas_claim("/proj/legacy", 1, "hash-new").unwrap();
    db.promote_cas_generation("/proj/legacy", 1).unwrap();
    let live = db.list_cas_live_refs().unwrap();
    assert_eq!(
        live,
        vec!["hash-new".to_string(), "hash-old".to_string()],
        "reused legacy hash survives promote (P0-A regression guard)"
    );
}
